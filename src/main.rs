mod sex_offender;
use std::error;
extern crate ftp;
use serde;

use crate::sex_offender::downloader::{Downloader, RecordInfo, FileInfo, DownloadOption, ExtractedFile};
use sex_offender::importer::{import_data, prepare_import};

use std::path;
use std::time::{Duration, Instant};
use core::borrow::Borrow;
use crate::sex_offender::importer::import_csv_file2;

//use crate::sexoffender::SexOffenderImportError;
//  FTP hostname:     ftptds.shadowsoft.com
//    username:             swg_eyemetric
//    password:             metric123swg99  
struct FtpSite {
    addr: String,
    user: String,
    pwd: String,
}
static FTP_ADDR: &'static str = "ftptds.shadowsoft.com:21";
static FTP_USER: &'static str = "swg_eyemetric";
static FTP_PWD: &'static str = "metric123swg99";

fn main() {

    println!("Begin Server Query phase. ");
    let start = Instant::now();
    let file_list = get_remote_files(); //all available files on remote server.
    //let flist: Vec<FileInfo> = file_list.into_iter().flatten().collect();
    //let jlist = serde_json::to_string(&flist);
    //all the files we haven't downloaded yet.
    //file names contain the date they were created and therefore updated offender data
    //for a state will be a name we don't have.
    let avail_updates = Downloader::available_updates(file_list);
    let duration = start.elapsed();
    println!("remote file listing complete. Took : {:?}", duration);

    println!("Begin Download Phase");

}

fn get_remote_files() -> Vec<Result<FileInfo, Box<error::Error>>> {
     let mut downloader = Downloader::connect(FTP_ADDR, FTP_USER, FTP_PWD).expect("Unable to connect to ftp server.");
    //set up some filters
    //we only want record and image files. The server has more that we don't use.
    let record_filter = |x: &String| x.contains("records") || x.contains("images");
    let records_only = |x: &String| x.contains("records");

    let az_only = |x: &String| x.contains("AR") && x.contains("records");

    let mut file_list = downloader.remote_file_list(record_filter, DownloadOption::Always);

/*
  let mut flist = &mut file_list;
    for file in flist.iter_mut() {
        match file {
            Ok(f) => {

                let arch = downloader.save_archive(&f);
                //let csv_files =Downloader::extract_archive(&f);
                println!("saved: {:?}", f.file_path().display());
            }
            Err(e) => {
                println!("could not read record! {:?}", e);
            }
        }
    }
    */
    file_list

}








