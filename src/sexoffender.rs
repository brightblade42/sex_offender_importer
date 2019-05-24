extern crate ftp;
use std::str;
use std::io::{Cursor};
use ftp::FtpStream;
use std::net::{SocketAddr, ToSocketAddrs};
use std::iter::Iterator;
use std::fmt;
use std::error::Error;

pub type Result<T> = ::std::result::Result<T, SexOffenderImportError>;

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

        let mut sex_offender_importer =  SexOffenderImporter {

            ftp_stream:  FtpStream::connect("ftptds.shadowsoft.com:21")
                .unwrap_or_else(|err| {
                panic!("{}", err);
            }),

        };

         sex_offender_importer.ftp_stream.login("swg_sample", "456_sample");

        sex_offender_importer

    }

    pub fn disconnect(&mut self) {
        self.ftp_stream.quit();
    }

    fn get_file_info(&self, path: &str, line: &str) -> Result<FileInfo> {

        let mut iter = line.split_whitespace().rev().take(5);

        let finfo = FileInfo {
            path: Some(path.to_string()),
            name : Some(iter.next().unwrap().to_string()),
            year : Some(iter.next().unwrap().to_string()),
            month : Some(iter.next().unwrap().to_string()),
            day : Some(iter.next().unwrap().to_string()),
            size : Some(iter.next().unwrap().to_string()),
        };

        Ok(finfo)

    }

    pub fn get_available_state_list(&mut self) -> Vec<Result<FileInfo>> {

        let sex_offender_path  = "/state/sex_offender";

        let lst = self.ftp_stream.nlst(Some("us")).unwrap();
        let available_states: Vec<Result<FileInfo>> = lst.into_iter().map(| p | {

        let mut pp = p.to_string();
        pp.push_str(sex_offender_path);

        //list the files available for this state
        //map_err?
       let res: Vec<Result<FileInfo>> = self.ftp_stream.list(Some(&pp))
           .into_iter()
           .flatten()
           .filter(|fi| fi.contains("records") || fi.contains("images"))
           .map(|fi| { self.get_file_info(&p, &fi)})
           .collect();

            res

        }).flatten().collect();


        available_states

    }

}

    struct SexOffenderFileInfo {
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
