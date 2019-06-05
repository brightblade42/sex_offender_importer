extern crate ftp;

use ftp::{FtpStream, FtpError};
use ftp::types::FileType;
use std::iter::{Iterator, FromIterator};
use std::fs::{self, File};
use std::path::{self, Path, PathBuf};
use std::str;
use std::io::{BufWriter, Write, Read, ErrorKind, BufReader, copy};
use std::convert::{AsRef, From};
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


pub enum DownloadOption {
    Only_New,
    Always,

}

impl Downloader {
    pub fn connect(addr: &str, user: &str, pwd: &str) -> Result<Self, Box<Error>> {
        //these values, I assume will be a configuration.

        let mut sex_offender_importer = Downloader {
            stream: FtpStream::connect(addr)?,
        };

        sex_offender_importer
            .stream
            .login(user, pwd);

        Ok(sex_offender_importer)
    }

    pub fn disconnect(&mut self) {
        self.stream.quit();
    }

    fn get_file_info(&self, path: &str, line: &str) -> Result<FileInfo, Box<Error>> {
        let mut iter = line.split_whitespace().rev().take(5);

        let fin = FileInfo::Record(RecordInfo {
            rpath: Some(path.to_string()),
            name: Some(iter.next().unwrap().to_string()),
            year: Some(iter.next().unwrap().to_string()),
            month: Some(iter.next().unwrap().to_string()),
            day: Some(iter.next().unwrap().to_string()),
            size: Some(iter.next().unwrap().to_string()),
        });

        Ok(fin)
    }


    ///returns a list of available files for download from remote server.
    ///a filter can be passed in to narrow the list.
    pub fn file_list(&mut self, filter: fn(&String) -> bool, file_opt: DownloadOption) -> Vec<Result<FileInfo, Box<Error>>> {

        //TODO: This list should be logged and checked against later when filtering on new files.
        let state_folders = self.stream.nlst(Some("us")).expect("Unable to get remote file listings");

        let on_file_option = |fi: &FileInfo| {
            match file_opt {
                DownloadOption::Always => true,
                DownloadOption::Only_New => Downloader::file_is_new(fi),
            }
        };


        let available_files: Vec<Result<FileInfo, Box<Error>>> = state_folders
            .into_iter()
            .map(|state_folder| {
                let sex_offender_folder = format!("{}{}", state_folder.to_string(), SEX_OFFENDER_PATH);
                let file_list: Vec<Result<FileInfo, Box<Error>>> = self.stream
                    .list(Some(&sex_offender_folder))
                    .into_iter()
                    .flatten()
                    .filter(filter)
                    .inspect(|line| println!("{}", line))
                    .map(|line| self.get_file_info(&state_folder, &line))
                    .filter(|fi| on_file_option(fi.as_ref().unwrap()))
                    .collect();

                file_list
            })
            .flatten()
            .collect();

        available_files
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


    fn remote_file_is_newer(fileinfo: &FileInfo) -> bool {
        //compare the local file mod time with fileinfo data
        false
    }
    ///downloads the remote archive file and writes to disk.
    pub fn save_archive(&mut self, fileinfo: &FileInfo) -> Result<SexOffenderArchive, Box<Error>> {
        let fname = fileinfo.name();//.as_ref().unwrap();
        //make sure we're setup to dload binary files
        self.stream.transfer_type(FileType::Binary)?;
        //change ftp dir.
        match self.stream.cwd(&fileinfo.remote_path().to_str().unwrap()) {
            Ok(()) => {
                println!("dir change success");

                let res = self.stream.retr(&fname, |stream| {
                    Downloader::write_archive(&fileinfo, stream);
                    Ok(())
                });

                ()
            }
            Err(e) => {
                println!("oops! {}", e);
            }
        }

        Ok(SexOffenderArchive {
            image: "none".to_string(),
            record: "none".to_string(),
        })
    }

    ///write the archive file we got from ftp to disk.
    fn write_archive(fileinfo: &FileInfo, stream: &mut Read) -> Result<usize, Box<Error>> {
        let mut file_path = fileinfo.file_path();
        let mut base_path = fileinfo.base_path();

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
    pub fn extract_archive(fileinfo: &FileInfo) -> Result<Vec<ExtractedFile>, Box<Error>> {
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

#[derive(Debug)]
pub enum ExtractedFile {
    //Csv(path::PathBuf),
    Csv { path: PathBuf, state: String },
    ImageArchive { path: PathBuf, state: String },
}


pub struct SexOffenderArchive {
    image: String,
    record: String,
}


#[derive(Debug)]
pub struct ImageInfo {
    pub rpath: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct RecordInfo {
    pub rpath: Option<String>,
    pub name: Option<String>,
    pub year: Option<String>,
    pub month: Option<String>,
    pub day: Option<String>,
    pub size: Option<String>, //convert this to i64
}


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
            rpath: Some("/home/d-rezzer/dev/ftp".to_string()),
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