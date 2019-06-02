mod sex_offender;
extern crate ftp;
use crate::sex_offender::downloader::{Downloader,  RecordInfo, FileInfo, DownloadOption, ExtractedFile};
use sex_offender::importer::import_data;

use std::path;
use std::time::{Duration, Instant};
use core::borrow::Borrow;
use crate::sex_offender::importer::import_csv_file2;

//use crate::sexoffender::SexOffenderImportError;
//

static FTP_ADDR: &'static str = "ftptds.shadowsoft.com:21";
static FTP_USER: &'static str = "swg_sample";
static FTP_PWD: &'static str = "456_sample";

fn main() {

    let start = Instant::now();
    get_remote_files();
    let duration = start.elapsed();
    println!("all done. Took : {:?}", duration);
}

fn get_remote_files() {
    let mut downloader = Downloader::connect(FTP_ADDR, FTP_USER, FTP_PWD).expect("Unable to connect to ftp server.");


    let record_filter = |x: &String | x.contains("records") || x.contains("images");
    let records_only = |x: &String| x.contains("records");
    let az_only = |x: &String| x.contains("AR") && x.contains("records");


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

    let mock_name = &mock_image.name();
    let mock_com = &mock_image.base_path().display().to_string();
    //let items = mock_com.split("/");


    flist.push(Ok(mock_image));
    use sex_offender::downloader::ExtractedFile::*;

     let ip = path::PathBuf::from("/home/d-rezzer/dev/ftp/us/arkansas/ARSexOffenders_2018_04_17_1726_images.zip");
     let imgarc = ExtractedFile::ImageArchive { path: ip, state: String::from(&mock_com[..2])};

     let mut cnt = 0;
    for file in flist.iter_mut() {
        match file {
            Ok(f) => {
                let mut ext = Downloader::extract_archive(&f);
                //create an image thign to test.
                for ef in ext.unwrap().iter() {
                    //import_data(&ef);
                    if let Csv {path,state} = ef {
                        println!("{:?}", path);
                        println!("{:?}", state);

                    } else {
                        println!("child zip: {:?}", ef);
                    }
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







