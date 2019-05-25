extern crate ftp;
use ftp::{FtpStream, FtpError};
use std::iter::Iterator;
use std::fs;
use std::path;
use std::str;
use std::io::{BufWriter, Write, ErrorKind};
use std::convert::AsRef;
use core::borrow::Borrow;
use std::process::id;
use std::error::{Error};
//use std::intrinsics::init;

static SEX_OFFENDER_PATH: &str = "/state/sex_offender";
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
                    .map(|fi| self.get_file_info(&p, &fi))
                    .collect();

                res
            })
            .flatten()
            .collect();

        available_files
    }


    pub fn get_archives(&mut self, fileinfo: &FileInfo) -> Result<SexOffenderArchive> {

        //download
        let p = fileinfo.path.as_ref().unwrap();
        let n = fileinfo.name.as_ref().unwrap();
        let fsiz = fileinfo.size.as_ref().unwrap();
        let remote_path = format!("/{}{}", p, SEX_OFFENDER_PATH);
        //change dir.
        match self.ftp_stream.cwd(&remote_path) {
            Ok(()) => {
                println!("dir change success");
                ()
            },
            Err(e) => {
                println!("oops! {}", e);

            }

        }

        use ftp::types::FileType;

        self.ftp_stream.transfer_type(FileType::Binary);

        let nsiz = fsiz.parse::<usize>().unwrap();

        let res = self.ftp_stream.retr(&n, |stream| {

            let mut local_path = String::from("/home/d-rezzer/dev/ftp/");
            local_path.push_str(n);
            let mut pth = path::Path::new(&local_path);
            let mut file = fs::File::create(pth).unwrap();

            let mut total_bytes:usize = 0;
            let mut buff: [u8; 4096] = [0;4096];

            println!("writing to path: {}", &local_path);

            let mut bytes_read = stream.read(&mut buff).unwrap();

            total_bytes += bytes_read;


            while bytes_read > 0 {

                let bytes_written = file.write(&buff[..bytes_read]).unwrap();

                bytes_read =  stream.read(&mut buff).unwrap();
                total_bytes += bytes_read;

                println!("bytes read: {}", bytes_read);
                println!("bytes written: {}", total_bytes);

            }


            match file.flush() {
                Ok(()) => {

                    println!("bytes written:: {}", total_bytes);
                },
                Err(e) => println!("bad mojo {}",e),
            }
            println!("according to size: {}", nsiz);

            Ok(())

        });

        Ok(SexOffenderArchive {

            image: "none".to_string(),
            record: "none".to_string(),
        })
    }

    //return the list of files that are newer than what we have
    fn filter_mod_time() {

    }
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
