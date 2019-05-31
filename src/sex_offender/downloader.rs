
extern crate ftp;
use ftp::{FtpStream, FtpError};
use ftp::types::FileType;
use std::iter::{Iterator, FromIterator};
use std::fs::{self, File};
use std::path::{self, Path};
use std::str;
use std::io::{BufWriter, Write, Read, ErrorKind, BufReader, copy};
use std::convert::AsRef;
use core::borrow::Borrow;
use std::process::id;
use std::{self, error::Error};
use zip;
use zip::ZipArchive;

static SEX_OFFENDER_PATH: &'static str = "/state/sex_offender";
static LOCAL_PATH: &'static str = "/home/d-rezzer/dev/ftp";

const CHUNK_SIZE: usize = 2048;

//pub type Result<T> = ::std::result::Result<T, Box<std::error::Error>>;

pub enum SexOffenderImportError {
    ConnectionError(std::io::Error),
    InvalidResponse(String),
    InvalidAddress(std::net::AddrParseError),
}

pub struct Downloader {
    stream: FtpStream,
}

impl Downloader {
    pub fn connect() -> Self {
        //these values, I assume will be a configuration.

        let mut sex_offender_importer = Downloader {
            stream: FtpStream::connect("ftptds.shadowsoft.com:21").unwrap_or_else(|err| {
                panic!("{}", err);
            }),
        };

        sex_offender_importer
            .stream
            .login("swg_sample", "456_sample");
        sex_offender_importer
    }

    pub fn disconnect(&mut self) {
        self.stream.quit();
    }

    fn get_file_info(&self, path: &str, line: &str) -> Result<FileInfo, Box<Error>> {
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




    pub fn file_list(&mut self, filter: fn(&String) -> bool ) -> Vec<Result<FileInfo, Box<Error>>> {

        let state_folders = self.stream.nlst(Some("us")).expect("Unable to get remote file listings");
        //TODO: Handle possible error
        let available_files: Vec<Result<FileInfo, Box<Error>>> = state_folders
            .into_iter()
            .map(|state_folder| {

                let sex_offender_folder = format!("{}{}", state_folder.to_string(), SEX_OFFENDER_PATH);
                let file_list: Vec<Result<FileInfo, Box<Error>>> = self.stream
                    .list(Some(&sex_offender_folder))
                    // .map_err(|e| e.into())  //TODO: don't presently know how to handle this properly
                    .into_iter()
                    .flatten()
                    .filter(filter)
                    .map(|fi| self.get_file_info(&state_folder, &fi))
                    //.filter(|fi| !Downloader::archive_exists(&fi.as_ref().unwrap()))
                    .filter(|fi| Downloader::file_is_new(fi.as_ref().unwrap()))
                    .collect();

                file_list
            })
            .flatten()
            .collect();

        available_files
    }

    fn file_is_new(fileinfo: &FileInfo) -> bool {
            //new means, remote file is newer or doesn't yet exist on local disk.
          !Downloader::archive_exists(fileinfo) || Downloader::remote_file_is_newer(fileinfo)
    }
    fn archive_exists(fileinfo: &FileInfo) -> bool {
        let exists = Path::new(&fileinfo.local_file_path()).exists();

        exists
    }

    fn remote_file_is_newer(fileinfo: &FileInfo) -> bool {
        //compare the local file mod time with fileinfo data
        false
    }

    pub fn get_archives(&mut self, fileinfo: &FileInfo) -> Result<SexOffenderArchive, Box<Error>> {

        let fname = fileinfo.name.as_ref().unwrap();
        //make sure we're setup to dload binary files
        self.stream.transfer_type(FileType::Binary)?;
        //change ftp dir.
        match self.stream.cwd(&fileinfo.remote_path()) {
            Ok(()) => {
                println!("dir change success");

                let res = self.stream.retr(&fname, |stream| {
                    Downloader::write_archive(&fileinfo, stream);
                        Ok(())
                });

                ()
            },
            Err(e) => {
                println!("oops! {}", e);
            }
        }


        Ok(SexOffenderArchive {
            image: "none".to_string(),
            record: "none".to_string(),
        })
    }


    fn write_archive(fileinfo: &FileInfo, stream: &mut Read) -> Result<usize, FtpError> {

        let fsize = fileinfo.size.as_ref().unwrap();
        let fsize = fsize.parse::<usize>().unwrap();
        let local_path = fileinfo.local_file_path();

        let mut local_path = Path::new(&local_path);
        let mut local_file = File::create(&local_path).expect("Unable to create archive file");

        let mut total_bytes: usize = 0;
        let mut buff: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

        let mut bytes_read = stream.read(&mut buff).expect("Unable to read bytes from stream");
        total_bytes += bytes_read;

        while bytes_read > 0 {
            let bytes_written = local_file.write(&buff[..bytes_read]).expect("Unable to write bytes");

            bytes_read = stream.read(&mut buff).expect("Unable to read bytes from stream");
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
        println!("according to size: {}", fsize);

        Ok(bytes_read)
    }
    //return the list of files that are newer than what we have
    fn filter_mod_time() {}


    fn extract_from(path: &path::Path) -> Result<(), Box<Error>> {

       let file = BufReader::new(File::open(path).unwrap());

           let mut archive = zip::ZipArchive::new(file)?;

           for i in 0 .. archive.len() {

              let mut arch_file = archive.by_index(i)?;

               let outname = arch_file.sanitized_name();
               let full_path = format!("{}/{}", LOCAL_PATH, outname.display());
               let mut outfile = BufWriter::new(File::create(&full_path).unwrap());
               std::io::copy(&mut arch_file, &mut outfile);
               println!("wrote: {}", full_path);

           }

        Ok(())
    }

    pub fn extract_archive(fileinfo: &FileInfo ) -> Result<Vec<CSVInfo>, Box<Error>> {

        let local_archive_path = fileinfo.local_file_path();
        let archived_file_name = Path::new(&local_archive_path);

        let mut csv_files: Vec<CSVInfo> = Vec::new(); //store our list of csv files.

        Downloader::extract_from(archived_file_name);

        Ok(csv_files)

    }
}


pub struct SexOffenderArchive {
    image: String,
    record: String,
}

#[derive(Debug)]
pub struct FileInfo {
    pub path: Option<String>,
    pub name: Option<String>,
    pub year: Option<String>,
    pub month: Option<String>,
    pub day: Option<String>,
    pub size: Option<String>, //convert this to i64
}

impl FileInfo {
    pub fn local_file_path(&self) -> String {
        format!("{}/{}", LOCAL_PATH, self.name.as_ref().unwrap())
    }

    pub fn remote_path(&self) -> String {
        format!("/{}{}", self.path.as_ref().unwrap(), SEX_OFFENDER_PATH)
    }
}

pub struct CSVInfo {
    pub name: String,
    pub path: String,

}
impl CSVInfo {

}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn extract_nested_zip_file() {
//        "/home/d-rezzer/dev/ftp/AZSX_2018_05_02_2355_records.zip"
        println!("TESTING!");
        let fileInfo = super::FileInfo {
            path: Some("/home/d-rezzer/dev/ftp".to_string()),
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