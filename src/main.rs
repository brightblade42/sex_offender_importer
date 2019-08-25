extern crate sex_offender;
extern crate log;

use std::error;

use std::error::Error;
extern crate ftp;

use serde;
use sex_offender::texas_shuffle;
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
use std::fs::{read_dir, File};
use std::{io, io::Write};
use mktemp::Temp;

use csv::{Reader, StringRecord, ReaderBuilder, Writer, WriterBuilder };
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



fn main_() {

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

fn main() {
    import_texas();
}
fn import_texas() -> Result<(), Box<dyn Error>> {
    let path_vars = PathVars::new(config::Env::Dev);
    let sql_path = PathBuf::from(&path_vars.vars["app_base_path"]).join(&path_vars.vars["sex_offender_db"]);
    //let mut root_path = PathBuf::from(&path_vars.vars["app_base_path"]).join(&path_vars.vars["archives_path"]);

//    let path_vars = PathVars::new(config::Env::Dev);
    let mut root_path = PathBuf::from(&path_vars.vars["app_base_path"]).join(&path_vars.vars["extracts_path"]);
    root_path.push("TX/records");

    let texas_files = vec![
        TexasFile {
            name: "Address.txt".into(),
            headers: vec!["AddressId", "IND_IDN", "SNU_NBR", "SNA_TXT", "SUD_COD", "SUD_NBR", "CTY_TXT", "PLC_COD", "ZIP_TXT", "COU_COD", "LAT_NBR", "LON_NBR"],
            raw: String::from("AddressId\tIND_IDN\tSNU_NBR\tSNA_TXT\tSUD_COD\tSUD_NBR\tCTY_TXT\tPLC_COD\tZIP_TXT\tCOU_COD\tLAT_NBR\tLON_NBR\n")
        }

        /*
        TexasFile {
            name: "BRTHDATE.txt".into(),
            headers: "DOB_IDN,PER_IDN,TYP_COD,DOB_DTE".into(),
        },
        TexasFile {
            name: "NAME.txt".into(),
            headers: "NAM_IDN,PER_IDN,TYP_COD,NAM_TXT,LNA_TXT,FNA_TXT".into(),
        },
        TexasFile {
            name: "OFF_CODE_SOR.txt".into(),
            headers: "COO_COD COJ_COD JOO_COD OFF_COD,VER_NBR,LEN_TXT,STS_COD,CIT_TXT,BeginDate,EndDate".into()
        },
        TexasFile {
            name: "Offense.txt".into(),
            headers: "IND_IDN,OffenseId,COO_COD,COJ_COD,JOO_COD,OFF_COD,VER_NBR,GOC_COD,DIS_FLG,OST_COD,CPR_COD,CDD_DTE,AOV_NBR SOV_COD,CPR_VAL ".into()
        },
        TexasFile {
            name: "Photo.txt".into(),
            headers: "IND_IDN,PhotoId,POS_DTE".into()
        },
        TexasFile {
            name: "PERSON.txt".into(),
            headers: "IND_IDN,PER_IDN,SEX_COD,RAC_COD,HGT_QTY,WGT_QTY,HAI_COD,EYE_COD,ETH_COD".into()
        }, */
    ];

    let conn = Connection::open(sql_path)?;
    let tx = State { state: "Texas".into(), abbr: "TX".into() };
    texas_files.iter().for_each(|tf| {

        let mut full_path = root_path.clone().join(&tf.name);
        let mut tab_list = String::new();

        let mut tlist: String = tf.headers.iter().fold(tab_list, |acc, head | {
            format!("{}\t{}",acc, head )
        });

        tlist.push_str("\n");
        let tlist = tlist.trim_start();

 //       println!("{}", tlist);



        prepend_file(tlist.as_bytes(), &full_path);

/*        let csv_file = File::open(&full_path).expect(&format!("Unable to open csv file {:?}", &full_path));
        let mut csv_reader = csv::ReaderBuilder::new()
            .delimiter('\t' as u8)
            .has_headers(false)
            .from_reader(csv_file);

        let sb = StringRecord::from(tf.headers.as_ref());
        csv_reader.set_headers(sb);
*/



    });

    Ok(())
}

   // extern crate mktemp;
    //use mktemp::Temp;

    fn prepend_file<P: AsRef<Path>>(data: &[u8], file_path: &P) -> io::Result<()> {
        // Create a temporary file
        let mut tmp_path = Temp::new_file()?;
        // Open temp file for writing

        // Stop the temp file being automatically deleted when the variable
        // is dropped, by releasing it.

         let tmp_path = tmp_path.release();


        let mut tmp = File::create(&tmp_path)?;
        // Open source file for reading
        let mut src = File::open(&file_path)?;
        // Write the data to prepend
        tmp.write_all(&data)?;
        // Copy the rest of the source file
        io::copy(&mut src, &mut tmp)?;
        fs::remove_file(&file_path)?;
        //fs::rename(&tmp_path, &file_path)?;
        fs::copy(&tmp_path, &file_path);
        fs::remove_file(&tmp_path);
        Ok(())
    }
        //prepare for import
/*
        match import_data(&xfile, sql_path.to_str().unwrap()) {
            Ok(()) => {
                println!("imported file {:?}", &exfile);
            }
            Err(e) => {
                println!("Error importing file {:?}", e);
                println!("ex: {:?}", &exfile);
            }
        }
*/




    //import data

struct TexasFile<'a> {
    name: String,
    headers: Vec<&'a str>,
    raw: String
}


fn fix_missing_headers(ext: &ExtractedFile) {
    match ext {
        ExtractedFile::Csv { path, state, delimiter} => {



        },
        _ => {
            println!("Not interesting")
        }

    }
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
