extern crate ftp;

use ftp::{FtpStream, FtpError};
use ftp::types::FileType;
use std::iter::{Iterator, FromIterator};
use std::fs;
use std::path;
use std::str;
use std::io::{BufWriter, Write, ErrorKind};
use std::convert::AsRef;
use core::borrow::Borrow;
use std::process::id;
use std::error::Error;

//use std::intrinsics::init;

static SEX_OFFENDER_PATH: &str = "/state/sex_offender";
static LOCAL_PATH: &str = "/home/d-rezzer/dev/ftp";
const CHUNK_SIZE: usize = 4096;

pub type Result<T> = ::std::result::Result<T, Box<std::error::Error>>;

pub enum SexOffenderImportError {
    ConnectionError(std::io::Error),
    InvalidResponse(String),
    InvalidAddress(std::net::AddrParseError),
}

pub struct SexOffenderImporter {
    ftp_stream: FtpStream,
}

impl SexOffenderImporter {
    pub fn connect() -> Self {
        //these values, I assume will be a configuration.

        let mut sex_offender_importer = SexOffenderImporter {
            ftp_stream: FtpStream::connect("ftptds.shadowsoft.com:21").unwrap_or_else(|err| {
                panic!("{}", err);
            }),
        };

        sex_offender_importer
            .ftp_stream
            .login("swg_sample", "456_sample");
        sex_offender_importer
    }

    pub fn disconnect(&mut self) {
        self.ftp_stream.quit();
    }

    fn get_file_info(&self, path: &str, line: &str) -> Result<FileInfo> {
        let mut iter = line.split_whitespace().rev().take(5);

        let finfo = FileInfo {
            path: Some(path.to_string()),
            name: Some(iter.next().unwrap().to_string()),
            year: Some(iter.next().unwrap().to_string()),
            month: Some(iter.next().unwrap().to_string()),
            day: Some(iter.next().unwrap().to_string()),
            size: Some(iter.next().unwrap().to_string()),
        };

        Ok(finfo)
    }


    /// returns a list of FileInfo for
    /// all record and image files for each available state.
    pub fn get_file_list(&mut self) -> Vec<Result<FileInfo>> {
        let sex_offender_path = "/state/sex_offender";

        let state_folders = self.ftp_stream.nlst(Some("us")).unwrap();
        let available_files: Vec<Result<FileInfo>> = state_folders
            .into_iter()
            .map(|p| {
                let mut pp = p.to_string();
                pp.push_str(sex_offender_path);

                //list the files available for this state
                //map_err?
                let res: Vec<Result<FileInfo>> = self.ftp_stream
                    .list(Some(&pp))
                    .into_iter()
                    .flatten()
                    .filter(|fi| fi.contains("records") || fi.contains("images"))
                    .map(|fi| {
                        self.get_file_info(&p, &fi)
                    }).filter(|fi|
                    {
                        !SexOffenderImporter::archive_exists(&fi.as_ref().unwrap())

                })
                    .collect();

                res
            })
            .flatten()
            .collect();

        available_files
    }


    fn archive_exists(fileinfo: &FileInfo) -> bool {
        let npath = format!("{}/{}", LOCAL_PATH, fileinfo.name.as_ref().unwrap());
        println!("local file path: {}", &npath);
        let exists = path::Path::new(&npath).exists();
        println!("pat exists: {}", exists);

        exists
    }


    pub fn get_archives(&mut self, fileinfo: &FileInfo) -> Result<SexOffenderArchive> {

        //download
        let p = fileinfo.path.as_ref().unwrap();
        let fname = fileinfo.name.as_ref().unwrap();
        let fsize = fileinfo.size.as_ref().unwrap();
        let fsize = fsize.parse::<usize>().unwrap();
        let remote_path = format!("/{}{}", p, SEX_OFFENDER_PATH);
        //change dir.
        match self.ftp_stream.cwd(&remote_path) {
            Ok(()) => {
                println!("dir change success");
                ()
            }
            Err(e) => {
                println!("oops! {}", e);
            }
        }

        //make sure we're setup to dload binary files
        self.ftp_stream.transfer_type(FileType::Binary);

        let res = self.ftp_stream.retr(&fname, |stream| {
            //let mut local_path = String::from(LOCAL_PATH);
           // local_path.push_str(fname);

            let npath = format!("{}/{}", LOCAL_PATH, fname);
            let mut pth = path::Path::new(&npath);
            let mut file = fs::File::create(pth).unwrap();

            let mut total_bytes: usize = 0;
            let mut buff: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

            let mut bytes_read = stream.read(&mut buff).unwrap();
            total_bytes += bytes_read;

            while bytes_read > 0 {
                let bytes_written = file.write(&buff[..bytes_read]).unwrap();

                bytes_read = stream.read(&mut buff).unwrap();
                total_bytes += bytes_read;

                println!("bytes read: {}", bytes_read);
                println!("bytes written: {}", total_bytes);
            }

            match file.flush() {
                Ok(()) => {
                    println!("bytes written:: {}", total_bytes);
                }
                Err(e) => println!("bad mojo {}", e),
            }
            println!("according to size: {}", fsize);

            Ok(())
        });

        Ok(SexOffenderArchive {
            image: "none".to_string(),
            record: "none".to_string(),
        })
    }


    //return the list of files that are newer than what we have
    fn filter_mod_time() {}
}

pub struct SexOffenderArchive {
    image: String,
    record: String,
}

#[derive(Debug)]
pub struct FileInfo {
    path: Option<String>,
    name: Option<String>,
    year: Option<String>,
    month: Option<String>,
    day: Option<String>,
    size: Option<String>, //convert this to i64
}
