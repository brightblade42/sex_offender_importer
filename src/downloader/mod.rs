extern crate ftp;
extern crate serde;

pub mod records;
pub mod archives;

use records::{RecordStatus, RecordInfo, FileInfo};

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
use rusqlite::{self, Connection, params, NO_PARAMS, ToSql};
use serde_rusqlite::{from_row, to_params_named, to_params};
use std::collections::HashSet;
use std::clone::Clone;
use crate::config::{self, Config, PathVars};
use std::ffi::OsStr;

use archives::SexOffenderArchive;

static IMPORT_LOG: &'static str = "/home/d-rezzer/dev/eyemetric/sex_offender/app/importlog.sqlite";
static SEX_OFFENDER_PATH: &'static str = "";
const CHUNK_SIZE: usize = 2048;

type GenError = Box<dyn std::error::Error>;
//type GenResult<T> = Result<T, GenError>;

pub type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Downloader {
    stream: FtpStream,
    config: PathVars,
    conn: Connection,
}

pub enum DownloadOption {
    Only_New,
    Always,
}

impl Downloader {

    ///create the Downloader object, connect and login to ftp server
    pub fn connect(addr: &str, user: &str, pwd: &str, config: PathVars) -> Result<Self>
    {
        //these values, I assume will be a configuration.

        let mut sex_offender_importer = Downloader {
            stream: FtpStream::connect(addr)?,
           config,
            conn: Connection::open(IMPORT_LOG).expect("A data connection"),
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
        println!("{}", ftp_line);
        let mut line = ftp_line.split_whitespace();
        //not sure about this.
        if line.clone().count() == 0 {
            return Err(GenError::from(format!("Unable to create FileInfo for {}", remote_file_path)));
        }
        let size = line.nth(4).unwrap();
        let name = line.last().unwrap();
        let date_str = self.get_date_from_name(name);

        let fin = FileInfo::Record(RecordInfo {
            rpath: Some(remote_file_path.to_string()),
            name: Some(name.to_string()),
            last_modified: Some(date_str),
            size: Some(size.parse().unwrap()),

            status: RecordStatus::None, //None is the beginning state.
        });

//        println!("{:?}", &fin);
        Ok(fin)
    }

    fn get_date_from_name(&self, name: &str) -> String {

            let mut nsplit: Vec<&str> = name.split('_').collect();
            nsplit.reverse();
            let y = nsplit.get(4).unwrap();
            let m = nsplit.get(3).unwrap();
            let d = nsplit.get(2).unwrap();


            format!("{}-{}-{}", y,m,d)

    }

    ///returns a list of available files for download from remote server.
    ///a filter can be passed in to narrow the list.
    pub fn get_updated_file_list(&mut self, filter: fn(&String) -> bool, file_opt: DownloadOption) -> Vec<Result<FileInfo>> {
        let paths = &self.config.vars; //TODO: consider loading once instead of every function call.
        let remote_base_path = &paths["ftp_base_path"];
        let sex_offender_path = &paths["ftp_sex_offender_path"].to_string();
        let state_folders = self.stream.nlst(Some(remote_base_path)).expect("Unable to get a remote file listing");

        println!("ftp paths");
        println!("{}", &remote_base_path);
        println!("{}", &sex_offender_path);

        let hard_filter = |x: &String| !x.contains(".txt") && !x.contains("united_states");

        let available_files: Vec<Result<FileInfo>> = state_folders
            .into_iter()
            .map(|state_folder| {
                let sex_offender_folder = format!("{}{}", state_folder.to_string(), sex_offender_path);
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

        self.log_available_updates(&available_files);


        available_files
    }

    ///returns a list of files that we have not tried to import previously.
    ///Note: it's possible that there are failed downloads in the log. We skip
    /// those possible downloads as they will be handled by a reschedule task (WIP).
    /// In other words, it's possible that the remote list contains files we want
    /// but since they failed to download previously we'll use a different mechanism
    /// to get them.
    ///
    pub fn log_available_updates(&self, remote_files: &Vec<Result<FileInfo>>) -> Result<()> {

        let res = self.conn
            .execute(
                "CREATE TABLE if not exists current_available (rpath, name, last_modified, size integer, status) ",
                NO_PARAMS,
            )
            .unwrap();

        self.conn.execute("BEGIN TRANSACTION", NO_PARAMS).expect("Unable to start transaction");

        for r_file in remote_files {
            if let Ok(FileInfo::Record(ri)) = r_file {

                self.conn.execute("DELETE FROM current_available where name=?",
                             &[ri.name.as_ref().unwrap()]);

                self.conn.execute_named(r#"INSERT INTO current_available (rpath, name, last_modified, size, status)
                                       VALUES (:rpath, :name, :last_modified, :size, :status )"#,
                                   &to_params_named(ri).unwrap().to_slice()).expect("Unable to insert row into temp table");
            }
        }

        self.conn.execute("COMMIT TRANSACTION", NO_PARAMS).expect("Failed to Commit Transaction!");

        Ok(())
    }

    fn log_download(&self, record_info: &RecordInfo) -> Result<()> {

        let rc = self.conn.execute("INSERT INTO  download_log (name, last_modified, size, bytes_downloaded, status) VALUES (?,?,?,?,?)",
                                         params![record_info.name, record_info.last_modified, record_info.size, 0, "InFlight" ])?;

        Ok(())
    }

    fn update_byte_count(&self, byte_count: i32, name: &str) -> Result<()>{
       let rc = self.conn.execute("UPDATE download_log set bytes_downloaded=? where name=?",
                        params![byte_count, name])?;

        Ok(())
    }
    pub fn download_file(&mut self, file_info: &FileInfo) -> Result<SexOffenderArchive> {

        println!("downloading {}  ... ", file_info.name());

        let res = self.save_archive(file_info).expect(&format!("Unable to complete saving archive {}", file_info.name()));

        println!("{:?}", file_info);

        if let FileInfo::Record(r) = file_info {
            self.log_download(&r);
        }

        Ok(res)

    }


    pub fn remote_path(&self, fileInfo: &FileInfo) -> PathBuf {
        let mut pb = match fileInfo {
            FileInfo::Record(r) => {
                r.rpath.as_ref().unwrap()
            }
            FileInfo::Image(i) => {
                i.rpath.as_ref().unwrap()
            }
        };


        let pb = PathBuf::from(format!("{}{}", pb, &self.config.vars["ftp_sex_offender_path"]));
        pb
    }

    pub fn local_archive_base(&self, fileinfo: &FileInfo) -> PathBuf {
        PathBuf::from(&self.config.vars["app_base_path"]).join(&self.config.vars["archives_path"])
        //PathBuf::from(&config.vars["archives_path"])
    }

    pub fn local_archive_path(&self, fileinfo: &FileInfo) -> PathBuf {
        let mut p = self.local_archive_base(fileinfo);
        p.push(fileinfo.name());
        p
    }

    fn create_download_log() {

    }

    fn update_download_log() {

    }
    ///downloads the remote archive file and writes to disk.


    pub fn save_archive(&mut self, fileinfo: &FileInfo) -> Result<SexOffenderArchive> {
        let fname = fileinfo.name();
        let local_archive_base = self.local_archive_base(fileinfo);
        let local_archive_file = self.local_archive_path(fileinfo );
        let local_archive_path = local_archive_file.clone();

        //make sure we're setup to dload binary files
        self.stream.transfer_type(FileType::Binary)?;
        //change ftp dir.
        let rpth = self.remote_path(&fileinfo);


        self.stream.cwd(rpth.to_str().unwrap()).expect("could not change ftp dir"); //.is_err() {
        println!("dir change success");

        let res = self.stream.retr(&fname, |stream| {

            fs::create_dir_all(&local_archive_base).expect("Unable to create archive dir");
            println!("local base dir: {}", local_archive_base.display());
            let mut local_file = BufWriter::new(File::create(&local_archive_file).expect("Unable to write archive to disk"));

            let mut total_bytes: usize = 0;
            let mut buff: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

            let mut bytes_read = stream.read(&mut buff).expect("Unable to read bytes");
            total_bytes += bytes_read;

            while bytes_read > 0 {
                let bytes_written = local_file.write(&buff[..bytes_read]).expect("Unable to write bytes");
                bytes_read = stream.read(&mut buff).expect("Unable to read stream");
                total_bytes += bytes_read
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
                path: local_archive_file.clone(),
                size: total_bytes,
            })
        });

        Ok(res.unwrap()) //workaround for error handling I don't understand.

    }

    //return the list of files that are newer than what we have
    fn filter_mod_time() {}
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct DownloadInfo {
    pub started: String,
    pub bytes_received: i32,
}

