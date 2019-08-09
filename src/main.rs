extern crate sex_offender;
extern crate log;

use std::error;

extern crate ftp;

use serde;

use sex_offender::types::{SexOffenderArchive, ExtractedFile, RecordInfo, FileInfo};
use sex_offender::downloader::{DownloadOption, Downloader};
use sex_offender::extractor::Extractor;
use sex_offender::importer::{import_data, prepare_import, delete_old_photos};

use core::borrow::Borrow;
use rusqlite::{params, Connection, NO_PARAMS};
use sex_offender::config::{self, Config, Env, FtpConfig, PathVars, States, State, LoadData};
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
    list: String,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() {

    //let path_config = config::PathVars::new(config::Env::Dev);
    //let ftp_conf = FtpConfig::init(config::Env::Test);
    //let addr = format!("{}:{}", &ftp_conf.address, &ftp_conf.port);
    //let args = Cli::from_args();
//    let args = SexOffenderCli::from_args();
    //   args.verbosity.setup_env_logger("sexoffenderimporter").unwrap();

    let statelist: States = config::States::load().unwrap();

    let path_vars = PathVars::new(config::Env::Dev);
    let sql_path = PathBuf::from(&path_vars.vars["app_base_path"]).join(&path_vars.vars["sex_offender_db"]);
    println!("sql path: {}", sql_path.to_str().unwrap());

    let mut root_path = PathBuf::from(&path_vars.vars["app_base_path"]).join(&path_vars.vars["archives_path"]);

    let statelist: States = config::States::load().unwrap();

    let path_vars = PathVars::new(config::Env::Dev);
    let sql_path = PathBuf::from(&path_vars.vars["app_base_path"]).join(&path_vars.vars["sex_offender_db"]);
    let mut root_path = PathBuf::from(&path_vars.vars["app_base_path"]).join(&path_vars.vars["archives_path"]);

    prepare_import(sql_path.to_str().unwrap());
    for state in statelist {
        println!("=================================");
        println!("{}: {}", state.state, state.abbr);

        println!("=================================");
        let state_path = root_path.join(state.state.to_lowercase());
        let st_files = read_dir(state_path).unwrap();

        delete_old_photos(state.abbr.as_str(), sql_path.to_str().unwrap());
        for stf in st_files {
            let fnn = stf.unwrap();
            println!("{:?}", fnn.path());
            //let conf = path_vars
            let sx = SexOffenderArchive::new(fnn.path(), 0);
            let mut ext = Extractor::new(&path_vars);

            let ef = ext.extract_archive(&sx).unwrap();
            println!("=================================");
            for exfile in ef {
                println!("Extract: {:?}", &exfile);
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
            println!("=================================");
        }

        println!("=================================");
    }

    println!("Dude, there's most of your data");
}

#[test]
fn connect_test() {
    let path_config = config::PathVars::new(config::Env::Production);
    let ftp_config = FtpConfig::init(config::Env::Production);

    assert_eq!(1, 0)
}

#[test]
fn disconnect_test() {
    assert_eq!(1, 0)
}

//#[test]
fn test_root_dirs() {
    let root_path = PathBuf::from("/home/d-rezzer/dev/eyemetric/ftp/us");
    let mut flist = fs::read_dir(root_path).unwrap();//.collect();

    for f in flist {
        let de = f.unwrap();
        println!("{:?}", de);
        for arch in fs::read_dir(de.path()).unwrap() {
            println!("==> {:?}", arch.unwrap());
        }
    }
}


fn get_remote_file_list(downloader: &mut Downloader) -> Vec<Result<FileInfo, Box<error::Error>>> {
    //set up some filters
    //we only want record and image files. The server has more that we don't use.
    let record_filter = |x: &String| x.contains("records") || x.contains("images");
    let records_only = |x: &String| x.contains("records");

    let az_only = |x: &String| x.contains("AR") && x.contains("records");
    let mut file_list = downloader.remote_file_list(record_filter, DownloadOption::Always);

    file_list
}
