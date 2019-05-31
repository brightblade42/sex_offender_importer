mod sex_offender;
extern crate ftp;
use crate::sex_offender::downloader::{Downloader, CSVInfo, FileInfo};
use sex_offender::importer::import_csv_file;
use std::path;
use std::time::{Duration, Instant};
use core::borrow::Borrow;

//use crate::sexoffender::SexOffenderImportError;
//
fn main() {

    let start = Instant::now();
    extract_nested_zip_file();
    //get_remote_files();
    let duration = start.elapsed();
    println!("all done. Took : {:?}", duration);
}

fn get_remote_files() {
    let mut downloader = Downloader::connect();


    let record_filter = |x: &String | x.contains("records") || x.contains("images");
//    let file_list = downloader.get_file_list();
//    let file_list = downloader.file_list(filter);
    let file_list = downloader.file_list(record_filter);
    let mut cnt = 0;

    if file_list.is_empty() {
        println!("There was nothing new to dload!");
    }
    for file in file_list {
        match file {
            Ok(f) => {
                println!("{:?}", f);
                   let arch = downloader.get_archives(&f);
                   let csv_files =Downloader::extract_archive(&f);

                   for cf in csv_files.unwrap() {
                       import_csv_file(path::Path::new(&cf.path)).expect("Unable to import csv file!");
                   }

                   println!("got an archive");
               }

            Err(e) => {
                println!("could not read record! {:?}", e);
            }
        }
    }

    downloader.disconnect();
}

fn extract_nested_zip_file() {
//        "/home/d-rezzer/dev/ftp/AZSX_2018_05_02_2355_records.zip"
    println!("TESTING!");
    let fileInfo = FileInfo {
        path: Some("/home/d-rezzer/dev/ftp".to_string()),
        name: Some("AZSX_2018_05_02_2355_images.zip".to_string()),
        year: None,
        month: None,
        day: None,
        size: None,
    };


 let fileInfo2 = FileInfo {
        path: Some("/home/d-rezzer/dev/ftp".to_string()),
        name: Some("AZSX_2018_05_02_2355_records.zip".to_string()),
        year: None,
        month: None,
        day: None,
        size: None,
    };

 let fileInfo3 = FileInfo {
        path: Some("/home/d-rezzer/dev/ftp".to_string()),
        name: Some("ARSexOffenders_2018_04_17_1726_images.zip".to_string()),
        year: None,
        month: None,
        day: None,
        size: None,
    };
let fileInfo4 = FileInfo {
        path: Some("/home/d-rezzer/dev/ftp/images".to_string()),
        name: Some("ARSexOffenders_images.1.zip".to_string()),
        year: None,
        month: None,
        day: None,
        size: None,
    };

//    let r = Downloader::extract_archive(&fileInfo3);
    let s = Downloader::extract_archive(&fileInfo4);
}





