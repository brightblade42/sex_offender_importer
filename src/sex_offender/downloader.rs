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

//TODO: get path info from ENV vars instead of hardcoding.
static SEX_OFFENDER_PATH: &'static str = "/state/sex_offender";
static LOCAL_PATH: &'static str = "/home/d-rezzer/dev/eyemetric/ftp";
static IMPORT_LOG: &'static str = "/home/d-rezzer/dev/eyemetric/ftp/importlog.sqlite";
const CHUNK_SIZE: usize = 2048;
type GenError = Box<std::error::Error>;
//type GenResult<T> = Result<T, GenError>;

pub type Result<T> = ::std::result::Result<T, Box<std::error::Error>>;

pub enum SexOffenderImportError {
    ConnectionError(std::io::Error),
    InvalidResponse(String),
    InvalidAddress(std::net::AddrParseError),
}

pub struct Downloader {
    stream: FtpStream,
}


pub enum DownloadOption {
    Only_New,
    Always,

}

impl Downloader {
    pub fn connect(addr: &str, user: &str, pwd: &str) -> Result<Self> {
        //these values, I assume will be a configuration.

        let mut sex_offender_importer = Downloader {
            stream: FtpStream::connect(addr)?,
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

        //TODO: This list should be logged and checked against later when filtering on new files.
        //top level state folders (ie us/new_jersey)
        let state_folders = self.stream.nlst(Some("us")).expect("Unable to get remote file listings");

        let on_file_option = |fi: &FileInfo| {
            match file_opt {
                DownloadOption::Always => true,
                DownloadOption::Only_New => Downloader::file_is_new(fi),
            }
        };

        let hard_filter = |x: &String| !x.contains(".txt") && !x.contains("united_states");

        let available_files: Vec<Result<FileInfo>> = state_folders
            .into_iter()
            .map(|state_folder| {
                let sex_offender_folder = format!("{}{}", state_folder.to_string(), SEX_OFFENDER_PATH);
                //println!("{}", &sex_offender_folder);

                let file_list: Vec<Result<FileInfo>> = self.stream
                    .list(Some(&sex_offender_folder)) //list remote dir
                    .into_iter()
                    .flatten()
                    .filter(hard_filter)
                    .filter(filter)
                    //.inspect(|line| println!("{}", line))
                    .map(|line| self.create_file_info(&state_folder, &line))
                    .filter(|fi| on_file_option(fi.as_ref().unwrap()))
                    .collect();

                file_list
            })
            .flatten()
            .collect();

        available_files
    }

    ///returns a list of files that we have not tried to import previously.
    ///Note: it's possible that there are failed downloads in the log. We skip
    /// those possible downloads as they will be handled by a reschedule task.
    /// In other words, it's possible that the remote list contains files we want
    /// but since they failed to download previously we'll use a different mechanism
    /// to get them.
    ///
    pub fn available_updates(remote_files: Vec<Result<FileInfo>>) -> Vec<FileInfo> {
        //get the difference between what we've previously imported and the remote files,
        //if any
        let conn = Connection::open(IMPORT_LOG).expect("unable to open a proper db connection");

        let res = conn
            .execute(
                "CREATE TABLE if not exists remote_file_list (rpath, name, last_modified, size integer, status) ",
                NO_PARAMS,
            )
            .unwrap();

        let res = conn.execute("CREATE TEMP TABLE if not exists remote_file_list_temp (rpath, name, last_modified, size integer, status);", NO_PARAMS)
            .expect("Unable to create remote_file_list_temp table");

        conn.execute("BEGIN TRANSACTION", NO_PARAMS).expect("Unable to start transaction");
        for r_file in remote_files {
            if let Ok(FileInfo::Record(ri)) = r_file {

                conn.execute_named("INSERT INTO remote_file_list_temp (rpath, name, last_modified, size, status) VALUES (:rpath, :name, :last_modified, :size, :status )",
                                   &to_params_named(ri).unwrap().to_slice()).expect("Unable to insert row into temp table");
            }
        }

        conn.execute("COMMIT TRANSACTION", NO_PARAMS).expect("Failed to Commit Transaction!");
        let mut stmt = conn.prepare("select * from remote_file_list_temp where name NOT IN (select name from remote_file_list) order by size")
            .expect("Unable to get an updated file listing");

        let tmp_import = stmt
            .query_and_then(NO_PARAMS, from_row::<RecordInfo>)
            .expect("Unable to get remote_file_list from Import Log");

        tmp_import.map(|r| FileInfo::Record(r.unwrap())).collect()

    }

    pub fn download_remote_files(&mut self, remote_file_list: Vec<FileInfo>) -> Vec<SexOffenderArchive> {

        let arch_list: Vec<SexOffenderArchive> = remote_file_list.into_iter().map(|r| {
            println!("downloading {} .... ", r.name());
            let res = self.save_archive(&r).expect("Unable to complete saving archive");

            if let FileInfo::Record(r) = r {
                self.log_download(&r);
            }

            res
        }).collect();

        arch_list
    }

    fn log_download(&self, record_info: &RecordInfo) {
        let conn = Connection::open(IMPORT_LOG).expect("unable to open a proper db connection");

        /*
        let name = record_info.name.as_ref().unwrap();
        let rpath = record_info.rpath.as_ref().unwrap();;
        let last_mod = record_info.last_modified.as_ref().unwrap();
        let size = record_info.size.as_ref().unwrap();
        let status = "Downloaded";
        */
        //params![photo_id,name,img_size, blob,state]).expect("A damn image import");
        //let rc = conn.execute("INSERT INTO remote_file_list (rpath, name, last_modified, size, status) VALUES (?,?, ?,?,?)",
          //                              params![rpath,name,last_mod, size, status]);


        let rc = conn.execute_named("INSERT INTO remote_file_list (rpath, name, last_modified, size, status) VALUES (:rpath, :name, :last_modified, :size, :status )",
                           &to_params_named(record_info).unwrap().to_slice()).expect("Unable to insert row into temp table");

        conn.execute("Update remote_file_list set status = 'Downloaded' where name=?",
                     &[record_info.name.as_ref().unwrap()]);

        //println!("logged download {:?}", record_info.name.unwrap() );

    }
    ///returns true if the file on the server is newer than what we have.
    fn file_is_new(fileinfo: &FileInfo) -> bool {
        //new means, remote file is newer or doesn't yet exist on local disk.
        !Downloader::archive_exists(fileinfo) || Downloader::remote_file_is_newer(fileinfo)
    }
    ///return true if we've previously downloaded the file archive.
    fn archive_exists(fileinfo: &FileInfo) -> bool {
        let exists = fileinfo.file_path().exists();
        println!("archive exists: {}", exists);
        exists
    }


//TODO: examine the last downloaded file list with the current one. update
//only newer files, replace file list "manifest"
    fn remote_file_is_newer(fileinfo: &FileInfo) -> bool {
        //compare the local file mod time with fileinfo data
        false
    }
    ///downloads the remote archive file and writes to disk.
    pub fn save_archive(&mut self, fileinfo: &FileInfo) -> Result<SexOffenderArchive> {
        let fname = fileinfo.name();//.as_ref().unwrap();

        //make sure we're setup to dload binary files
        self.stream.transfer_type(FileType::Binary)?;
        //change ftp dir.

        let size: usize = match self.stream.cwd(&fileinfo.remote_path().to_str().unwrap()) {
            Ok(()) => {
                println!("dir change success");

                let res = self.stream.retr(&fname, |stream| {
                   let sz = Downloader::write_archive(&fileinfo, stream).expect("Unable to write archive!");
                    Ok(sz)
                });

                (0)
            }
            Err(e) => (0)

        };

        Ok(SexOffenderArchive {
            path: fileinfo.file_path().clone(),
            size,
        })
    }

    ///write the archive file we got from ftp to disk.
    fn write_archive(fileinfo: &FileInfo, stream: &mut Read) -> Result<usize> {
        let file_path = fileinfo.file_path();
        let base_path = fileinfo.base_path();

        fs::create_dir_all(&base_path)?;
        println!("local base dir: {}", base_path.display());
        let mut local_file = BufWriter::new(File::create(&file_path)?);

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

        Ok(bytes_read)
    }
    //return the list of files that are newer than what we have
    fn filter_mod_time() {}

    ///extracts an arcive into a list of ExtractedFile types.
    /// An Extracted file can be one of two variants. Csv or ImageArchive
    pub fn extract_archive(fileinfo: &FileInfo) -> Result<Vec<ExtractedFile>> {
        let mut extracted_files: Vec<ExtractedFile> = Vec::new(); //store our list of csv files.
        let archive_path = fileinfo.file_path();
        let file = BufReader::new(File::open(archive_path)?);

        let mut archive = zip::ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut arch_file = archive.by_index(i)?;

            let outname = arch_file.sanitized_name();
            let mut fp = fileinfo.extract_path();
            fs::create_dir_all(fp.as_path())?;
            fp.push(outname);
            let mut outfile = BufWriter::new(File::create(fp.as_path())?);
            std::io::copy(&mut arch_file, &mut outfile)?;
            //println!("wrote: {}", fp.display());
            let tag = &fileinfo.name()[..2];

            extracted_files.push(
                match fileinfo {
                    FileInfo::Record(_) => ExtractedFile::Csv { path: fp, state: String::from(tag) },
                    FileInfo::Image(_) => ExtractedFile::ImageArchive { path: fp, state: String::from(tag) }
                });
        }

        Ok(extracted_files)
    }
}

#[derive(Debug, Serialize,Deserialize)]
pub enum ExtractedFile {
    //Csv(path::PathBuf),
    Csv { path: PathBuf, state: String },
    ImageArchive { path: PathBuf, state: String },
}


pub struct SexOffenderArchive {
    path: PathBuf,
    size: usize,
}


#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct ImageInfo {
    pub rpath: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct DownloadInfo {
    pub started: String,
    pub bytes_received: i32,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum RecordStatus {
    None,
    InFlight,
    Failed,
    Downloaded,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct RecordInfo {
    pub rpath: Option<String>,
    pub name: Option<String>,
    pub last_modified: Option<String>,
    pub size: Option<i64>, //convert this to i64
    pub status: RecordStatus,
}


#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
#[serde(tag = "type")]
pub enum FileInfo {
    Record(RecordInfo),
    Image(ImageInfo),
}

impl FileInfo {
    pub fn name(&self) -> String {
        use FileInfo::*;
        match *self {
            Record(ref r) => r.name.as_ref().unwrap().to_string(),
            Image(ref i) => i.name.as_ref().unwrap().to_string(), //as_ref().unwrap()
        }
    }

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

    pub fn remote_path(&self) -> path::PathBuf {
        use FileInfo::*;

        path::PathBuf::from(
            match *self {
                Record(ref r) => format!("/{}{}", r.rpath.as_ref().unwrap(), SEX_OFFENDER_PATH),
                Image(ref i) => format!("/{}{}", i.rpath.as_ref().unwrap(), SEX_OFFENDER_PATH),
            }
        )
    }

    pub fn extract_path(&self) -> path::PathBuf {
        use FileInfo::*;

        let mut fp = self.base_path();

        match *self {
            Record(ref r) => fp.push("records"),
            Image(ref i) => fp.push("images"),
        };

        fp
    }
}


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
