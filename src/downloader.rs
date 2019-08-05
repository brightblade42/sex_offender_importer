
extern crate ftp;
extern crate serde;
use ftp::{FtpStream, FtpError};
use ftp::types::FileType;
use std::iter::{Iterator, FromIterator};
use std::fs::{self, File};
use std::path::{self, Path, PathBuf};
use std::str;
use std::io::{BufWriter, Write, Read, ErrorKind, BufReader, copy};
use std::convert::{AsRef, From};
use core::borrow::{Borrow, BorrowMut};
use std::process::id;
use std::{self, error::Error};
use zip;
use zip::ZipArchive;
use serde_derive::{Serialize, Deserialize};
use serde_json;
use rusqlite::{self, Connection,params,  NO_PARAMS, ToSql};
use serde_rusqlite::{from_row, to_params_named};
use std::collections::HashSet;
use std::clone::Clone;
use crate::config::{self, Config, PathVars};
use std::ffi::OsStr;
use super::types::{SexOffenderArchive, RecordInfo, RecordStatus, FileInfo};

static IMPORT_LOG: &'static str = "/home/d-rezzer/dev/eyemetric/ftp/importlog.sqlite";
const CHUNK_SIZE: usize = 2048;


type GenError = Box<dyn std::error::Error>;
//type GenResult<T> = Result<T, GenError>;

pub type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;



pub enum SexOffenderImportError {
    ConnectionError(std::io::Error),
    InvalidResponse(String),
    InvalidAddress(std::net::AddrParseError),
}

pub struct Downloader {
    stream: FtpStream,
    config: PathVars,
}


pub enum DownloadOption {
    Only_New,
    Always,

}

impl Downloader {
    pub fn connect(addr: &str, user: &str, pwd: &str, config: PathVars) -> Result<Self>
     {
        //these values, I assume will be a configuration.

        let mut sex_offender_importer = Downloader {
            stream: FtpStream::connect(addr)?,
            config,
        };

        sex_offender_importer
            .stream
            .login(user, pwd).expect("Unable to login to ftp site");

        Ok(sex_offender_importer)
    }

    pub fn disconnect(&mut self) {
        self.stream.quit().expect("Unable to quite ftp Stream");
    }


    fn create_file_info(&self, remote_file_path: &str, ftp_line: &str) -> Result<FileInfo> {
        let mut line = ftp_line.split_whitespace();
        //not sure about this.
        if line.clone().count() == 0 {
            return Err(GenError::from(format!("Unable to create FileInfo for {}", remote_file_path)));
        }
        let size = line.nth(4).unwrap();
        let name = line.last().unwrap();
        let nsplit: Vec<&str> = name.split('_').collect();

        let y = nsplit.get(1).unwrap();
        let m = nsplit.get(2).unwrap();
        let d = nsplit.get(3).unwrap();

        let fin = FileInfo::Record(RecordInfo {
            rpath: Some(remote_file_path.to_string()),
            name: Some(name.to_string()),
            last_modified: Some(format!("{}-{}-{}", y, m, d)),
            size: Some(size.parse().unwrap()),

            status: RecordStatus::None, //None is the beginning state.
        });

//        println!("{:?}", &fin);
        Ok(fin)
    }


    ///returns a list of available files for download from remote server.
    ///a filter can be passed in to narrow the list.
    pub fn remote_file_list(&mut self, filter: fn(&String) -> bool, file_opt: DownloadOption) -> Vec<Result<FileInfo>> {

       let paths =  &self.config.vars; //TODO: consider loading once instead of every function call.
       let remote_base_path = &paths["remote_base_path"];
        let sex_offender_path = &paths["sex_offender_path"].to_string();
        let state_folders = self.stream.nlst(Some(remote_base_path)).expect("Unable to get a remote file listing");

        let hard_filter = |x: &String| !x.contains(".txt") && !x.contains("united_states");

        let available_files: Vec<Result<FileInfo>> = state_folders
            .into_iter()
            .map(|state_folder| {

                let sex_offender_folder = format!("{}{}", state_folder.to_string(),sex_offender_path);
               // println!("{}", &sex_offender_folder);

                let file_list: Vec<Result<FileInfo>> = self.stream
                    .list(Some(&sex_offender_folder)) //list remote dir
                    .into_iter()
                    .flatten()
                    .filter(hard_filter)
                    .filter(filter)
                    //.inspect(|line| println!("{}", line))
                    .map(|line| self.create_file_info(&state_folder, &line))
                    .collect();

                file_list
            })
            .flatten()
            .collect();

        available_files
    }

    ///returns a list of files that we have not tried to import previously.
    ///Note: it's possible that there are failed downloads in the log. We skip
    /// those possible downloads as they will be handled by a reschedule task (WIP).
    /// In other words, it's possible that the remote list contains files we want
    /// but since they failed to download previously we'll use a different mechanism
    /// to get them.
    ///
    pub fn available_updates(remote_files: Vec<Result<FileInfo>>) -> Vec<FileInfo> {
        //get the difference between what we've previously imported and the remote files,
        //if any
        let conn = Connection::open(IMPORT_LOG).expect("unable to open a proper db connection");

        //table to store a log of available archives and the status of their downloads
        let res = conn
            .execute(
                "CREATE TABLE if not exists remote_file_list (rpath, name, last_modified, size integer, status) ",
                NO_PARAMS,
            )
            .unwrap();

        //Temp table to hold a new file listing to compare to any existing remote file listing.
        let res = conn.execute("CREATE TEMP TABLE if not exists remote_file_list_temp (rpath, name, last_modified, size integer, status);", NO_PARAMS)
            .expect("Unable to create remote_file_list_temp table");

        //let res = conn.execute("CREATE TABLE if not exists remote_file_list_temp (rpath, name, last_modified, size integer, status);", NO_PARAMS)
         //   .expect("Unable to create remote_file_list_temp table");

        conn.execute("BEGIN TRANSACTION", NO_PARAMS).expect("Unable to start transaction");

        for r_file in remote_files {
            if let Ok(FileInfo::Record(ri)) = r_file {

                conn.execute_named(r#"INSERT INTO remote_file_list_temp (rpath, name, last_modified, size, status)
                                       VALUES (:rpath, :name, :last_modified, :size, :status )"#,
                                   &to_params_named(ri).unwrap().to_slice()).expect("Unable to insert row into temp table");
            }
        }

        conn.execute("COMMIT TRANSACTION", NO_PARAMS).expect("Failed to Commit Transaction!");
        //get file listing that do NOT exist in the remote_file_list table. These are fresh fishies.
        //The very first time there will be nothing in the remote_file_list table, only the remote_file_list_temp table.
        let mut stmt = conn.prepare(r#"SELECT * from remote_file_list_temp
                                        WHERE name NOT IN (select name from remote_file_list)
                                        order by size"#)
                            .expect("Unable to get an updated file listing");

        let tmp_import = stmt
            .query_and_then(NO_PARAMS, from_row::<RecordInfo>)
            .expect("Unable to get remote_file_list from Import Log");

        //return the freshest of the freshies.
        tmp_import.map(|r| FileInfo::Record(r.unwrap())).collect()

    }
///cool
    pub fn download_remote_files(&mut self, remote_file_list: Vec<FileInfo>) -> Vec<SexOffenderArchive> {

        let arch_list: Vec<SexOffenderArchive> = remote_file_list.into_iter().map(|r| {
            println!("downloading {} .... ", r.name());
            let res = self.save_archive(&r).expect("Unable to complete saving archive");

            println!("{:?}", &r);
            if let FileInfo::Record(r) = r {
                self.log_download(&r);
            }

            res
        }).collect();

        arch_list
    }

    fn log_download(&self, record_info: &RecordInfo) {
        let conn = Connection::open(IMPORT_LOG).expect("unable to open a proper db connection");

        let rc = conn.execute_named("INSERT INTO remote_file_list (rpath, name, last_modified, size, status) VALUES (:rpath, :name, :last_modified, :size, :status )",
                           &to_params_named(record_info).unwrap().to_slice()).expect("Unable to insert row into temp table");

        conn.execute("Update remote_file_list set status = 'Downloaded' where name=?",
                     &[record_info.name.as_ref().unwrap()]);

    }
    ///returns true if the file on the server is newer than what we have.
    /*fn file_is_new(fileinfo: &FileInfo) -> bool {
        //new means, remote file is newer or doesn't yet exist on local disk.
        !Downloader::archive_exists(fileinfo) || Downloader::remote_file_is_newer(fileinfo)
    }
    ///return true if we've previously downloaded the file archive.
    fn archive_exists(fileinfo: &FileInfo) -> bool {
        let exists = fileinfo.file_path().exists();
        println!("archive exists: {}", exists);
        exists
    }
    */


//TODO: examine the last downloaded file list with the current one. update
//only newer files, replace file list "manifest"
    fn remote_file_is_newer(fileinfo: &FileInfo) -> bool {
        //compare the local file mod time with fileinfo data
        false
    }

    fn remote_ftp_path(&mut self, fileinfo: &FileInfo) -> PathBuf {

        let p = match fileinfo {
            FileInfo::Record(r) => {
                PathBuf::from(r.rpath.as_ref().unwrap())
            }
            _ => PathBuf::new(),
        };

       p

    }
    pub fn remote_path(fileInfo: &FileInfo, config: &PathVars) -> PathBuf {

        let mut pb = match fileInfo {
            FileInfo::Record(r) => {
                r.rpath.as_ref().unwrap()
            },
            FileInfo::Image(i) => {
                i.rpath.as_ref().unwrap()
            }
        };

        let pb = PathBuf::from(format!("{}{}", pb, &config.vars["sex_offender_path"] ));
       pb
    }

    pub fn local_archive_base(fileinfo: &FileInfo, config: &PathVars ) -> PathBuf {
        PathBuf::from(&config.vars["local_archive_path"])
    }

    pub fn local_archive_path(fileinfo: &FileInfo, config: &PathVars) -> PathBuf {

        let mut p = Downloader::local_archive_base(fileinfo, config);
        p.push(fileinfo.name());
        p
    }

    ///downloads the remote archive file and writes to disk.

    pub fn save_archive(&mut self, fileinfo: &FileInfo) -> Result<SexOffenderArchive> {
        let fname = fileinfo.name();

        let conf= &self.config;
        let local_archive_path = Downloader::local_archive_path(fileinfo, conf);
        //make sure we're setup to dload binary files

        self.stream.transfer_type(FileType::Binary)?;
        //change ftp dir.
        let rpth = Downloader::remote_path(&fileinfo, conf);//&fileinfo.remote_path();

        let size: usize = match self.stream.cwd(rpth.to_str().unwrap()) {
            Ok(()) => {
                println!("dir change success");
                let res = self.stream.retr(&fname, |stream| {
                   let sz = Downloader::write_archive(&fileinfo, stream, conf).expect("Unable to write archive!");
                    Ok(sz)
                });

                (0)
            }
            Err(e) => {
                println!("could not change ftp dir {:?} {}", rpth, e);
                (0)
            }

        };

        Ok(SexOffenderArchive {
            path: local_archive_path, //fileinfo.file_path().clone(),
            size,
        })
    }

    ///write the archive file we got from ftp to disk.
    fn write_archive(fileinfo: &FileInfo, stream: &mut Read, config: &PathVars) -> Result<SexOffenderArchive> {

        let local_archive_base = Downloader::local_archive_base(fileinfo, config);
        let local_archive_file = Downloader::local_archive_path(fileinfo, config);

        fs::create_dir_all(&local_archive_base)?;
        println!("local base dir: {}", local_archive_base.display());
        let mut local_file = BufWriter::new(File::create(&local_archive_file)?);

        let mut total_bytes: usize = 0;
        let mut buff: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

        let mut bytes_read = stream.read(&mut buff)?;
        total_bytes += bytes_read;

        while bytes_read > 0 {
            let bytes_written = local_file.write(&buff[..bytes_read])?;
            bytes_read = stream.read(&mut buff)?;
            total_bytes += bytes_read;
        }
        //TODO: if the file size from the server doesn't match
        //the total bytes we've received then we didn't get the whole file
        //das a problem. I wonder if the server supports ftp resume.
        match local_file.flush() {
            Ok(()) => {
                println!("bytes written:: {}", total_bytes);
            }
            Err(e) => println!("bad mojo {}", e),
        }


        Ok(SexOffenderArchive {
            path: local_archive_file,
            size: total_bytes,
        })
    }
    //return the list of files that are newer than what we have
    fn filter_mod_time() {}


}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct DownloadInfo {
    pub started: String,
    pub bytes_received: i32,
}


/*
    pub fn base_path(&self) -> path::PathBuf {
        use FileInfo::*;

        path::PathBuf::from(
            match *self {
                Record(ref r) => format!("{}/{}", LOCAL_PATH, r.rpath.as_ref().unwrap()),
                Image(ref i) => format!("{}/{}", LOCAL_PATH, i.rpath.as_ref().unwrap()),
            }
        )
    }

    pub fn file_path(&self) -> path::PathBuf {
        use FileInfo::*;

        let mut fp = self.base_path();
        match *self {
            Record(ref r) => fp.push(r.name.as_ref().unwrap()),
            Image(ref i) => fp.push(i.name.as_ref().unwrap()),
        };

        fp
    }
*/

    /*
    pub fn extract_path(&self) -> path::PathBuf {
        use FileInfo::*;

        let mut fp = self.base_path();

        match *self {
            Record(ref r) => fp.push("records"),
            Image(ref i) => fp.push("images"),
        };

        fp
    } */


/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_nested_zip_file() {
//        "/home/d-rezzer/dev/ftp/AZSX_2018_05_02_2355_records.zip"
        println!("TESTING!");
        let fileInfo = super::FileInfo {
            rpath: Some("/home/d-rezer/dev/eyemetric/ftp".to_string()),
            name: Some("AZSX_2018_05_02_2355_images.zip".to_string()),
            year: None,
            month: None,
            day: None,
            size: None,
        };

        let s = Downloader::extract_archive(&fileInfo);
        assert_eq!(true, true);
    }
}
*/
