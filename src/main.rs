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
    let start = Instant::now();
    let file_list = get_remote_files();
    //let flist: Vec<FileInfo> = file_list.into_iter().flatten().collect();
    //let jlist = serde_json::to_string(&flist);
    let avail_updates = Downloader::available_updates(file_list);
    //println!("{:?}", jlist );
    let duration = start.elapsed();
    println!("all done. Took : {:?}", duration);
}
fn get_remote_files() -> Vec<Result<FileInfo, Box<error::Error>>> {
     let mut downloader = Downloader::connect(FTP_ADDR, FTP_USER, FTP_PWD).expect("Unable to connect to ftp server.");
    //set up some filters
    let record_filter = |x: &String| x.contains("records") || x.contains("images");
    let records_only = |x: &String| x.contains("records");
    let az_only = |x: &String| x.contains("AR") && x.contains("records");

    let mut file_list = downloader.remote_file_list(record_filter, DownloadOption::Always);
    //file_list.serialize();
    let mut cnt = 0;

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
fn get_remote_files2() {

    let mut downloader = Downloader::connect(FTP_ADDR, FTP_USER, FTP_PWD).expect("Unable to connect to ftp server.");
    let record_filter = |x: &String| x.contains("records") || x.contains("images");
    let records_only = |x: &String| x.contains("records");
    let az_only = |x: &String| x.contains("AR") && x.contains("records");

     
    let mut file_list = downloader.remote_file_list(record_filter, DownloadOption::Always);
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
    /*
    use sex_offender::downloader::ImageInfo;
    let mock_image = FileInfo::Image(ImageInfo {
        rpath: Some("us/arkansas".to_string()),
        name: Some("ARSexOffenders_2018_04_17_1726_images.zip".to_string()),
    });

    let mock_name = &mock_image.name();
    let mock_com = &mock_image.base_path().display().to_string();
    flist.push(Ok(mock_image));
    use sex_offender::downloader::ExtractedFile::*;
    let ip = path::PathBuf::from("/home/d-rezer/dev/eyemetric/ftp/us/arkansas/ARSexOffenders_2018_04_17_1726_images.zip");
    let imgarc = ExtractedFile::ImageArchive { path: ip, state: String::from(&mock_com[..2]) };
    let mut cnt = 0;
    prepare_import();

    for file in flist.iter_mut() {
        match file {
            Ok(f) => {
                let mut ext = Downloader::extract_archive(&f);
                //create an image thign to test.
                for ef in ext.unwrap().iter() {
                    import_data(&ef);
                    if let Csv { path, state } = ef {
                        println!("{:?}", path);
                        println!("{:?}", state);
                    } else {
                        println!("child zip: {:?}", ef);
                    }
                }
            }
            Err(err) => {
                println!("I have no idea what i am doing!");
            }
        }
    }

    */

    downloader.disconnect();
}







