#![allow(dead_code)]
extern crate sex_offender;
extern crate log;

use std::error;
extern crate ftp;

use sex_offender::importer::{ Import, ExtractedFile,
                              import_data,
                              prepare_import,
//                              delete_old_photos
};
use sex_offender::downloader::archives::SexOffenderArchive;
use sex_offender::downloader::{ Downloader,
                               //DownloadInfo,
                               DownloadOption,
                               records::FileInfo};
use sex_offender::extractors::Extractor;

//use rusqlite::{params, Connection, NO_PARAMS};
use sex_offender::config::{self, PathVars, States, LoadData, FtpConfig};
use std::path::{ PathBuf};
//use std::time::{Duration, Instant};
use std::fs;
use quicli::prelude::*;
use structopt::StructOpt;
use std::fs::{read_dir};
//use std::{io, io::Write};
//use mktemp::Temp;

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

fn get_sql_path(vars: &PathVars) -> PathBuf {

    let sql_path = PathBuf::from(&vars.vars["app_base_path"]).join(&vars.vars["sex_offender_db"]);
    println!("sql path: {}", sql_path.to_str().unwrap());
    sql_path
}

fn get_root_path(vars: &PathVars) -> PathBuf {
     PathBuf::from(&vars.vars["app_base_path"]).join(&vars.vars["archives_path"])
}

fn download_files() {

    //maybe just download the enchilada?
    //let avail; //entire list
    //let avail_updates; //new than what we already have

    let ftp_conf = FtpConfig::init(config::Env::Production);
    let addr = "ftptds.shadowsoft.com:21";
    let user = "swg_eyemetric";
    let pass = "metric123swg99";

    //println!("{}", &ftp_conf.address);
    let path_vars = PathVars::new(config::Env::Production);


    let mut downloader = Downloader::connect(addr, user, pass ,
                                             path_vars).expect("Unable to get a good connection");

    let record_filter = |x: &String| x.contains("records") || x.contains("images");

    //let record_filter = |x: &String| x.contains("records");
    let file_list = downloader.get_updated_file_list(record_filter, DownloadOption::Always);
    let file_list = downloader.get_newest_update_list();
    println!("oh hello");

    let sex_offender_archives =  file_list.iter()
        .map(|fi| {
            let m = downloader.download_file(&fi.as_ref().unwrap());
            println!("downloaded: {}", fi.as_ref().unwrap().name());
            m
        });

    for arch in sex_offender_archives {

        if let Ok(sx) = arch {
            println!("all good");
            //do something
        } else {
            println!("not good at all.");
            //err, bad download, add to error queue?
        }
    }

}
fn main() {

    //download_files();
    //import_files();


    let statelist: States = config::States::load().unwrap();
    //let path_vars = PathVars::new(config::Env::Dev);

    //let statelist: States = config::States::load().unwrap();
   // let statelist = statelist.iter().filter(|s| s.abbr == "NJ");
    let statelist = statelist.iter().filter(|s| s.abbr == "TX");
    let path_vars = PathVars::new(config::Env::Production);
    let sql_path = get_sql_path(&path_vars);
    let  root_path = get_root_path(&path_vars);

    let _prep_result = prepare_import(sql_path.to_str().unwrap());

    for state in statelist {
        println!("=================================");
        println!("{}: {}", state.state, state.abbr);

        println!("=================================");
        let state_path = root_path.join(state.abbr.to_uppercase());
        let st_files = read_dir(state_path).expect("A file but got us a directory");

        //delete_old_photos(state.abbr.as_str(), sql_path.to_str().unwrap());
        for stf in st_files {
            let fnn = stf.unwrap();
            println!("{:?}", fnn.path());
            //let conf = path_vars
            let sx = SexOffenderArchive::new(fnn.path(), 0);
            let mut ext = Extractor::new(&path_vars);
            let skip_images = true; //time consuming during test phase.

            let ef: Vec<ExtractedFile> = ext.extract_archive(&sx, skip_images).expect("A file but got a directory");
            println!("=================================");
            for exfile in ef {

                exfile.import().expect("Unable to complete file import");
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

fn main_() {

    //let path_config = config::PathVars::new(config::Env::Dev);
    //let ftp_conf = FtpConfig::init(config::Env::Test);
    //let addr = format!("{}:{}", &ftp_conf.address, &ftp_conf.port);
    //let args = Cli::from_args();
//    let args = SexOffenderCli::from_args();
    //   args.verbosity.setup_env_logger("sexoffenderimporter").unwrap();

    let statelist: States = config::States::load().unwrap();
    //let path_vars = PathVars::new(config::Env::Dev);

    //let statelist: States = config::States::load().unwrap();
    let statelist = statelist.iter().filter(|s| s.abbr == "AK");
    let path_vars = PathVars::new(config::Env::Dev);
    let sql_path = get_sql_path(&path_vars);
    let  root_path = get_root_path(&path_vars);

    let _prep_result = prepare_import(sql_path.to_str().unwrap());

    for state in statelist {
        println!("=================================");
        println!("{}: {}", state.state, state.abbr);

        println!("=================================");
        let state_path = root_path.join(state.state.to_lowercase());
        let st_files = read_dir(state_path).expect("A file but got us a directory");

        //delete_old_photos(state.abbr.as_str(), sql_path.to_str().unwrap());
        for stf in st_files {
            let fnn = stf.unwrap();
            println!("{:?}", fnn.path());
            //let conf = path_vars
            let sx = SexOffenderArchive::new(fnn.path(), 0);
            let mut ext = Extractor::new(&path_vars);
            let skip_images = true; //time consuming during test phase.

            let ef: Vec<ExtractedFile> = ext.extract_archive(&sx, skip_images).expect("A file but got a directory");
            println!("=================================");
            for exfile in ef {

                exfile.import().expect("Unable to complete file import");
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
    let flist = fs::read_dir(root_path).unwrap();//.collect();

    for f in flist {
        let de = f.unwrap();
        println!("{:?}", de);
        for arch in fs::read_dir(de.path()).unwrap() {
            println!("==> {:?}", arch.unwrap());
        }
    }
}


fn fix_directories() {

    let statelist: States = config::States::load().unwrap();
    let path_vars = PathVars::new(config::Env::Production);
    let  root_path = get_root_path(&path_vars);

    for state in statelist {
        let npath = root_path.join(state.abbr.to_uppercase());
        if let Ok(()) = fs::create_dir(&npath) {
            println!("made dir : {:?}", &npath);
        } else {
            println!("path already exists, no need to create");
        }

        println!("=======================================================");
        for entry in fs::read_dir(&root_path).unwrap() {
            let entry = entry.unwrap();
            let data = entry.metadata().unwrap();
            let path = entry.path();

            if data.is_file() {
                if let Some(ex) = path.extension() {
                    let pre = &entry.file_name().to_os_string().into_string().unwrap().to_uppercase()[0..2];
                    //println!("pre: {}", pre);
                    if ex == "zip" && pre == &state.abbr.to_uppercase() {
                        println!("{} length {}", path.display(), data.len());
                    }
                }
            }
        }
        println!("=======================================================");

    }

}
