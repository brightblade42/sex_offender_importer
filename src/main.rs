mod sex_offender;
extern crate ftp;
use crate::sex_offender::downloader::{Downloader, CSVInfo, RecordInfo, FileInfo, DownloadOption, ExtractedFile};
use sex_offender::importer::import_csv_file;
use std::path;
use std::time::{Duration, Instant};
use core::borrow::Borrow;

//use crate::sexoffender::SexOffenderImportError;
//
fn main() {

    let start = Instant::now();
    //extract_nested_zip_file();
    get_remote_files();
    let duration = start.elapsed();
    println!("all done. Took : {:?}", duration);
}

fn get_remote_files() {
    let mut downloader = Downloader::connect();


    let record_filter = |x: &String | x.contains("records") || x.contains("images");
    let records_only = |x: &String| x.contains("records");
    let az_only = |x: &String| x.contains("AR") && x.contains("records");
//    let file_list = downloader.get_file_list();
//    let file_list = downloader.file_list(filter);

//    let file_list = downloader.file_list(record_filter, FileOption::Only_New);
    let mut file_list = downloader.file_list(records_only, DownloadOption::Always);
    let mut cnt = 0;

    if file_list.is_empty() {
        println!("There was nothing new to dload!");
    }
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

    use sex_offender::downloader::ImageInfo;

    let mock_image = FileInfo::Image(ImageInfo {
        rpath: Some("us/arkansas".to_string()),
        name: Some("ARSexOffenders_2018_04_17_1726_images.zip".to_string()),
    });

    flist.push(Ok(mock_image));

    use sex_offender::downloader::ExtractedFile::*;

     let ip = path::PathBuf::from("/home/d-rezzer/dev/ftp/us/arkansas/ARSexOffenders_2018_04_17_1726_images.zip");
     let imgarc = ExtractedFile::ImageArchive(ip);
     let mut cnt = 0;
    for file in flist.iter_mut() {
        match file {
            Ok(f) => {
                let mut ext = Downloader::extract_archive(&f);
                //create an image thign to test.
                for ef in ext.unwrap().iter() {
                    if let Csv(c) = ef {
                        println!("{:?}", c);

                    } else {
                        println!("child zip: {:?}", ef);
                    }
                    /*match ef {
                        Csv(c) => println!("{}", c)
                    }*/
//                for ef in ext.iter() {
                    //println!("extracted: {:?}", ef);
                }
            },
            Err(err) => {
                println!("I have no idea what i am doing!");
            }
        }
    }

    //need to test image extraction without downloading.

    /*
                     let csv_files =Downloader::extract_archive(&f);

                   for cf in csv_files.unwrap() {
                       import_csv_file(path::Path::new(&cf.path)).expect("Unable to import csv file!");
                   }


    */


    downloader.disconnect();
}

fn extract_nested_zip_file() {
//        "/home/d-rezzer/dev/ftp/AZSX_2018_05_02_2355_records.zip"
    println!("TESTING!");
    let fileInfo = RecordInfo {
        rpath: Some("/home/d-rezzer/dev/ftp".to_string()),
        name: Some("AZSX_2018_05_02_2355_images.zip".to_string()),
        year: None,
        month: None,
        day: None,
        size: None,
    };


 let fileInfo2 = RecordInfo {
        rpath: Some("/home/d-rezzer/dev/ftp".to_string()),
        name: Some("AZSX_2018_05_02_2355_records.zip".to_string()),
        year: None,
        month: None,
        day: None,
        size: None,
    };

 let fileInfo3 = RecordInfo {
        rpath: Some("/home/d-rezzer/dev/ftp".to_string()),
        name: Some("ARSexOffenders_2018_04_17_1726_images.zip".to_string()),
        year: None,
        month: None,
        day: None,
        size: None,
    };
let fileInfo4 = RecordInfo {
        rpath: Some("/home/d-rezzer/dev/ftp/images".to_string()),
        name: Some("ARSexOffenders_images.1.zip".to_string()),
        year: None,
        month: None,
        day: None,
        size: None,
    };

//    let r = Downloader::extract_archive(&fileInfo3);
    let s = Downloader::extract_archive(&FileInfo::Record(fileInfo4));
}





