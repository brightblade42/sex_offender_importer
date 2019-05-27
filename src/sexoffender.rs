extern crate ftp;
use ftp::{FtpStream, FtpError};
use ftp::types::FileType;
use std::iter::{Iterator, FromIterator};
use std::fs;
use std::path;
use std::str;
use std::io::{BufWriter, Write, Read, ErrorKind, BufReader, copy};
use std::convert::AsRef;
use core::borrow::Borrow;
use std::process::id;
use std::{self, error::Error};
use zip;
static SEX_OFFENDER_PATH: &'static str = "/state/sex_offender";
static LOCAL_PATH: &'static str = "/home/d-rezzer/dev/ftp";

const CHUNK_SIZE: usize = 2048;

//pub type Result<T> = ::std::result::Result<T, Box<std::error::Error>>;

pub enum SexOffenderImportError {
    ConnectionError(std::io::Error),
    InvalidResponse(String),
    InvalidAddress(std::net::AddrParseError),
}

pub struct SexOffenderDownloader {
    ftp_stream: FtpStream,
}

impl SexOffenderDownloader {
    pub fn connect() -> Self {
        //these values, I assume will be a configuration.

        let mut sex_offender_importer = SexOffenderDownloader {
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


    /// returns a list of FileInfo for
    /// all record and image files for each available state.
    pub fn get_file_list(&mut self) -> Vec<Result<FileInfo, Box<Error>>> {

        let state_folders = self.ftp_stream.nlst(Some("us")).unwrap(); //TODO: Handle possible error
        let available_files: Vec<Result<FileInfo, Box<Error>>> = state_folders
            .into_iter()
            .map(|state_folder| {

                let sex_offender_folder = format!("{}{}", state_folder.to_string(), SEX_OFFENDER_PATH);
                let file_list: Vec<Result<FileInfo, Box<Error>>> = self.ftp_stream
                    .list(Some(&sex_offender_folder))  //TODO: map_err?
                    .into_iter()
                    .flatten()
                    .filter(|fi| fi.contains("records") || fi.contains("images"))
                    .map(|fi| self.get_file_info(&state_folder, &fi))
                    .filter(|fi| !SexOffenderDownloader::archive_exists(&fi.as_ref().unwrap()))
                    .collect();

                file_list
            })
            .flatten()
            .collect();

        available_files
    }


    fn archive_exists(fileinfo: &FileInfo) -> bool {
        let exists = path::Path::new(&fileinfo.local_file_path()).exists();

        exists
    }


    pub fn get_archives(&mut self, fileinfo: &FileInfo) -> Result<SexOffenderArchive, Box<Error>> {

        let fname = fileinfo.name.as_ref().unwrap();
        //make sure we're setup to dload binary files
        self.ftp_stream.transfer_type(FileType::Binary)?;
        //change ftp dir.
        match self.ftp_stream.cwd(&fileinfo.remote_path()) {
            Ok(()) => {
                println!("dir change success");
                ()
            }
            Err(e) => {
                println!("oops! {}", e);
            }
        }

        let res = self.ftp_stream.retr(&fname, |stream| {
            SexOffenderDownloader::write_archive(&fileinfo, stream);
            Ok(())
        });


        Ok(SexOffenderArchive {
            image: "none".to_string(),
            record: "none".to_string(),
        })
    }


    fn write_archive(fileinfo: &FileInfo, stream: &mut Read) -> Result<usize, FtpError> {


        let fsize = fileinfo.size.as_ref().unwrap();
        let fsize = fsize.parse::<usize>().unwrap();
        let local_path = fileinfo.local_file_path();

        let mut local_path = path::Path::new(&local_path);
        let mut local_file = fs::File::create(&local_path).unwrap();

        let mut total_bytes: usize = 0;
        let mut buff: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

        let mut bytes_read = stream.read(&mut buff).unwrap();
        total_bytes += bytes_read;

        while bytes_read > 0 {
            let bytes_written = local_file.write(&buff[..bytes_read]).unwrap();

            bytes_read = stream.read(&mut buff).unwrap();
            total_bytes += bytes_read;

            println!("bytes read: {}", bytes_read);
            println!("bytes written: {}", total_bytes);
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

    pub fn extract_archive(fileinfo: &FileInfo ) -> Result<(), Box<Error>> {
        let local_path = fileinfo.local_file_path();
        let file_name = path::Path::new(&local_path);

        let file = fs::File::open(&file_name)?;
        let mut archive = zip::ZipArchive::new(file)?;

        for i in 0 .. archive.len() {
            let mut file = archive.by_index(i)?;

            let outpath = file.sanitized_name();
            let extracted_file_path = format!("{}/{}", LOCAL_PATH, file.name());
            let mut final_path = path::Path::new(&extracted_file_path);
            //a little block
            {
                let comment = file.comment();
                if !comment.is_empty() {
                    println!("File {} comment: {}", i, comment);
                }
            }


            //&*file.name  what is that syntax?
            if (&*file.name()).ends_with('/') {
                println!("File {} extracted to \"{}\"", i, outpath.as_path().display());
                fs::create_dir_all(&outpath).unwrap();
            } else {
                println!("File {} extracted to \"{}\" ({} bytes)", i, final_path.display(), file.size());
                /*if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p).unwrap();
                    }
                }
                */
                let mut outfile = fs::File::create(&final_path).unwrap();
                std::io::copy(&mut file, &mut outfile).unwrap();
            }
        }

        Ok(())

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

impl FileInfo {
    pub fn local_file_path(&self) -> String {
        format!("{}/{}", LOCAL_PATH, self.name.as_ref().unwrap())
    }

    pub fn remote_path(&self) -> String {

        //let p = fileinfo.path.as_ref().unwrap();
        format!("/{}{}", self.path.as_ref().unwrap(), SEX_OFFENDER_PATH)
    }
}

pub struct ArchiveExtractor {

}
