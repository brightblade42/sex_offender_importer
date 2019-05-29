mod sex_offender;
extern crate ftp;
use crate::sex_offender::downloader::{Downloader, CSVInfo};
use sex_offender::importer::import_csv_file;
use std::path;
//use crate::sexoffender::SexOffenderImportError;
//
fn main() {
    let mut downloader =Downloader::connect();

    let file_list = downloader.get_file_list();
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

    println!("all done");
}







