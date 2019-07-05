extern crate sex_offender;
use std::error;
extern crate ftp;
use serde;

use sex_offender::downloader::{Downloader, RecordInfo, FileInfo, DownloadOption, ExtractedFile};
use sex_offender::importer::{import_data, prepare_import};

use std::path;
use std::time::{Duration, Instant};
use core::borrow::Borrow;
use crate::sex_offender::importer::import_csv_file2;
use rusqlite::{Connection, params, NO_PARAMS};
use sex_offender::config::{self, FtpConfig, Config, Env, PathVars};
use std::path::{PathBuf, Path};
use std::collections::HashMap;


fn main() {

    let pv = config::PathVars::new(config::Env::Dev);

    let ftp_conf =  FtpConfig::init(config::Env::Test);
    let addr = format!("{}:{}",&ftp_conf.address, &ftp_conf.port);

    let mut downloader = Downloader::connect(&addr, &ftp_conf.user, &ftp_conf.pass, pv).expect("to connect to ftp server.");

    println!("Begin Server Query phase. ");
    let start = Instant::now();

    let file_list = get_remote_file_list(&mut downloader); //FileInfo vec
    //println!("{:?}", file_list);

    let duration = start.elapsed();
    println!("remote file listing complete. Took : {:?}", duration);

    let avail_updates = Downloader::available_updates(file_list);

   //let top_one = avail_updates.into_iter().take(2).collect();



    println!("Begin Download Phase");
    //let sx_arch_files = downloader.download_remote_files(top_one);
    let sx_arch_files = downloader.download_remote_files(avail_updates);

    prepare_import();
    for sx_file in sx_arch_files {
        let exfiles = downloader.extract_archive2(sx_file);
        let exfileL = match exfiles {
            Ok(ext) => {
                ext
            },
            Err(e) => {
                println!("BAD MOJO! {}", e);
                continue; //skip bad data. we'll log it. tag it and bag it.
            }
        };
        for exfile in exfileL { //.expect("Bad exfiles") {
            match import_data(&exfile) {
                Ok(()) => {
                    println!("imported file {:?}", &exfile);
                }
                Err(e) => {
                    println!("Error importing file {:?}", e);
                    println!("ex: {:?}", &exfile);
                }
            }
        }

       // println!("ex: {:?}", exfile);
    }


    //println!("{:?}",sx_arch);
    println!("End of line...");
downloader.disconnect();

}

fn get_remote_file_list(downloader: &mut Downloader) -> Vec<Result<FileInfo, Box<error::Error>>> {
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