extern crate sex_offender;
extern crate log;
use std::error;
extern crate ftp;
use serde;

use sex_offender::types::{SexOffenderArchive, ExtractedFile, RecordInfo, FileInfo };
use sex_offender::downloader::{DownloadOption, Downloader,   };
use sex_offender::extractor::{Extractor};
use sex_offender::importer::{import_data, prepare_import};

use core::borrow::Borrow;
use rusqlite::{params, Connection, NO_PARAMS};
use sex_offender::config::{self, Config, Env, FtpConfig, PathVars};
use std::collections::HashMap;
use std::path;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::fs;
use quicli::prelude::*;
use structopt::StructOpt;
use std::ffi::{OsStr, OsString};
use std::fs::read_dir;

#[derive(Debug, StructOpt)]
struct SexOffenderCli {
    #[structopt(long = "count", short = "n", default_value = "3")]
    count: usize,

    #[structopt(long = "extract", short = "x", default_value = "none")]
    extract: String,
    #[structopt(long = "import", short = "i", default_value = "new")]
    import: String,
    #[structopt(long = "download", short = "d", default_value = "new")]
    download: String,

    #[structopt(long = "list", short = "l", default_value = "new")]
    list:String,
    #[structopt(flatten)]
    verbosity: Verbosity,
}
fn main()  {

    //let path_config = config::PathVars::new(config::Env::Dev);
    //let ftp_conf = FtpConfig::init(config::Env::Test);
    //let addr = format!("{}:{}", &ftp_conf.address, &ftp_conf.port);
    //let args = Cli::from_args();
//    let args = SexOffenderCli::from_args();
 //   args.verbosity.setup_env_logger("sexoffenderimporter").unwrap();

    let path_vars = PathVars::new(config::Env::Dev);
    let sql_path = PathBuf::from(&path_vars.vars["app_base_path"]).join(&path_vars.vars["sex_offender_db"]);
    //println!("{:?}", path_vars.vars);
    println!("sql path: {}", sql_path.to_str().unwrap());
   //let mut root_path = PathBuf::from(&path_vars.vars["archives_path"]);

    let mut root_path = PathBuf::from(&path_vars.vars["app_base_path"]).join(&path_vars.vars["archives_path"]);
 //   if args.extract == "alabama" {
       //root_path.push("alabama");
//    root_path.push("arkansas");
    root_path.push("alaska");

    println!("hello");
        let st_files = read_dir(root_path).unwrap();

        prepare_import(sql_path.to_str().unwrap());
        for stf in st_files {
            let fnn = stf.unwrap();
            println!("{:?}", fnn.path());
            //let conf = path_vars
            let sx = SexOffenderArchive::new(fnn.path(), 0);
            let mut ext = Extractor::new(&path_vars);


                let ef = ext.extract_archive(&sx).unwrap();
                for exfile in ef {
                    match import_data(&exfile, sql_path.to_str().unwrap()) {
                        Ok(()) => {
                            println!("imported file {:?}", &exfile);
                        }
                        Err(e) => {
                            println!("Error importing file {:?}", e);
                            println!("ex: {:?}", &exfile);
                        }
                    }
                }

        }

    println!("HELP! For fooks sake");
}

#[test]
fn connect_test() {

    let path_config = config::PathVars::new(config::Env::Production);
    let ftp_config = FtpConfig::init(config::Env::Production);

   assert_eq!(1,0)
}

#[test]
fn disconnect_test() {
   assert_eq!(1,0)
}

//#[test]
fn test_root_dirs() {
    let root_path = PathBuf::from("/home/d-rezzer/dev/eyemetric/ftp/us");
    //let iter = fs::read_dir(root_path);
    let mut flist = fs::read_dir(root_path).unwrap();//.collect();

    for f in flist {
        let de = f.unwrap();
        println!("{:?}", de);
        for arch in fs::read_dir(de.path()).unwrap() {
            println!("==> {:?}", arch.unwrap());
        }

    }
}

fn extract_state_archive() {

    let path_vars = config::PathVars::new(config::Env::Dev);

}
fn main_old() {
    let path_config = config::PathVars::new(config::Env::Dev);
    let ftp_conf = FtpConfig::init(config::Env::Test);

    //let path_config = config::PathVars::new(config::Env::Production);
    //let ftp_conf =  FtpConfig::init(config::Env::Production);

    let addr = format!("{}:{}", &ftp_conf.address, &ftp_conf.port);

    let mut downloader = Downloader::connect(&addr, &ftp_conf.user, &ftp_conf.pass, path_config)
        .expect("to connect to ftp server.");

    println!("Begin Server Query phase. ");
    let start = Instant::now();

    let file_list = get_remote_file_list(&mut downloader); //all the files we could get
                                                           //println!("{:?}", file_list);

    let duration = start.elapsed();
    println!("remote file listing complete. Took : {:?}", duration);

    let avail_updates = Downloader::available_updates(file_list); //the files we need
    println!("available file count: {}", &avail_updates.len());
    //let avail_updates: Vec<FileInfo> = avail_updates.into_iter().take(1).collect();
    for upd in &avail_updates {
        println!(
            "hello: {}",
            &format!(
                "name: {}   path: {:?}",
                upd.name().as_str(),
                upd.remote_path().to_str()
            )
        );
    }

    println!("Begin Download Phase");
    //let sx_arch_files = downloader.download_remote_files(top_one);
    //let sx_arch_files = downloader.download_remote_files(avail_updates);

    //println!("Finished downloading... ready for extraction");

    //    prepare_import();

    /*
    for sx_file in sx_arch_files {
        let exfiles = downloader.extract_archive(sx_file);
        let exfileL = match exfiles {
            Ok(ext) => {
                ext
            },
            Err(e) => {
                println!("BAD MOJO! {}", e);
                continue; //skip bad data. we'll log it. tag it and bag it.
            }
        };

       //there can be many files stored in the archive. loop em!
        for exfile in exfileL {
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
    */
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
