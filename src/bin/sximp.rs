
#![allow(dead_code)]

//use std::time::{Duration, Instant};
//use quicli::prelude::*;
use structopt::StructOpt;
use indicatif::{ProgressStyle, ProgressBar};
use console::{self, style};
#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};
use std::{fs::{self, read_dir}, path::PathBuf, thread};

use sex_offender::{
    importer::{self, Import, ExtractedFile },
    downloader::{
        Downloader,
        DownloadOption,

    },
    extractors::Extractor,
    config::{self, PathVars, States, LoadData, FtpConfig},
};
use sex_offender::extractors::ExtractOptions;
use std::time::Duration;
use sex_offender::util::{GenResult, GenError};
use sex_offender::downloader::records::FileInfo;
use std::borrow::Borrow;

/*#[derive(Debug, StructOpt)]
#[structopt(name= "sx-imp", about = "cli tool for downloading sex offender data")]
struct SexOffenderCli {
    #[structopt(long = "count", short = "n", default_value = "3")]
    count: usize,
    #[structopt(long = "extract", short = "x", default_value = "none")]
    extract: String,
    #[structopt(long = "import", short = "i", default_value = "new")]
    import: String,
    #[structopt(long = "download", short = "d", default_value = "new")]
    download: String,

    //pull from local db unless upgrade option set? like a lock file?
    #[structopt(long = "list", short = "l", default_value = "new")]
    list: String,
}
*/

#[derive(Debug, StructOpt)]
#[structopt(name= "sx-imp", about = "cli tool for downloading and managing sex offender data",
author="ryan lee martin")]
enum SXOCli {

    #[structopt(name="get")]
    ///retrieve sex offender archives from public data
    ///
    ///An archive is considered new if it hasn't been downloaded yet
   Get {
      #[structopt(short,long)]
      ///pass -a to begin downloading any newly available archives
      new: bool,
       #[structopt(short,long )]
       ///pass -s state_name to download a specific state archive
       state: Option<String>,
   },
    #[structopt(name="list")]
    ///list available archive files
    ///
    ///Archives are periodically updated with new sex offender information.
    ///Sex offender data is charged by the download so we want to make sure we only
    ///download what is necessary.
    List {

        #[structopt(short,long )]
        ///pass -n to list any new archives ready for download.
        new: bool,
        #[structopt(short,long)]
        unchanged: bool,
        #[structopt(short,long)]
        ///pass -a to list all available archives. Included those that have been downloaded.
        all: bool
    },

    #[structopt(name="build")]
    ///build a download log from previously downloaded archive session. You probably won't use this.
    ///
    ///download log is created while data is downloading but if something isn't right
    ///we can rebuild the log.
    Build {
        #[structopt(long="dlog")]
        ///pass -dlog to rebuild the download log from archives on disk. snoogans.
        download_log: bool,
    }
}

fn main()  {

    let mut downloader: Downloader = create_downloader();
//    let opt = SexOffenderCli::from_args();
    let opt = SXOCli::from_args();

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ])
            .template("{spinner:.blue} {msg}"),
    );

    match opt {

        SXOCli::List { new, ..} if new => {

            let new_list = get_new_list(&mut downloader);

            let mut table = Table::new();
            table.add_row(row!["FILES READY FOR DOWNLOAD", &format!("total: {}",new_list.len())]);
            table.add_row(row!["path", "file name"]);

            for item in new_list {

                let f = item.unwrap();
                let path = f.remote_path().display().to_string();
                let name = f.name();
                table.add_row(row![path,name]);
            }

            table.printstd();
        },
        SXOCli::List { unchanged, .. } if unchanged => {
           pb.set_message("Retrieving unchanged list..");
            let unchanged_list = get_unchanged_list(&mut downloader);

            let mut table = Table::new();
            table.add_row(row!["Unchanged", &format!("total: {}",unchanged_list.len())]);
            table.add_row(row!["path", "file name"]);

            for item in unchanged_list {

                let f = item.unwrap();
                let path = f.remote_path().display().to_string();
                let name = f.name();
                table.add_row(row![path,name]);
            }

            table.printstd();
        }
        SXOCli::List { all, ..} if all => {

            pb.set_message("Retrieving list of all files...");
            let all_list = get_all_avail(&mut downloader);

            let mut table = Table::new();
            table.add_row(row!["ALL AVAIL", &format!("total: {}",all_list.len())]);
            table.add_row(row!["path", "file name"]);

            for item in all_list {

                let f = item.unwrap();
                let path = f.remote_path().display().to_string();
                let name = f.name();
                table.add_row(row![path,name]);
            }

            table.printstd();

        },

        SXOCli::Build { download_log } if download_log => {

            pb.set_message("rebuilding download log...");
            thread::sleep(Duration::from_millis(500));
            match downloader.rebuild_log_from_archives() {
                Ok(x) => {
                    pb.finish_with_message("Download logs have been rebuilt")
                },
                Err(e) => {
                    pb.finish_with_message(&format!("There was a problem rebuilding the logs! {}",e));
                }
            }
        },

        SXOCli::Get { new, state} => {
            if new {

                let file_list =  get_new_list(&mut downloader);//dloader.get_newest_update_list();

               // let file_list = file_list.iter().filter(|f| f.unwrap().name())
                pb.set_message("Retrieving available archives...");
                let sex_offender_archives =  file_list.iter()
                    .map(|fi| {
                        let info = fi.as_ref().unwrap();
                        pb.set_message(&format!("Downloading {}",info.name() ));

                        let m = downloader.download_file(info);

                        println!("downloaded: {}", info.name());
                        m
                    });
                pb.finish_with_message("Done downloading archives! Ready to import.");
            } else if let Some(s) = state {

                let file_list = get_new_list(&mut downloader );

                let file_list = file_list.iter().filter(|f| f.as_ref().unwrap().name()[..2] == "AL".to_string());

                for f in file_list {

                    let info = f.as_ref().unwrap();
                    pb.set_message(&format!("Retrieving {}..", info.name()));
                    let m = downloader.download_file(info);
                }

                pb.finish_with_message("Done with requested files");
            }
        },

        _ => { println!("not ready for that one chief");}
    }

}

fn get_new_list(dloader: &mut Downloader) -> Vec<GenResult<FileInfo>>{

    let record_filter = |x: &String| x.contains("records");
    dloader.get_newest_available_list()

}

fn get_unchanged_list(dloader: &mut Downloader) -> Vec<GenResult<FileInfo>> {
    dloader.get_unchanged_list()
}

fn get_all_avail(dloader: &mut Downloader) -> Vec<GenResult<FileInfo>> {

    let file_filter = |x: &String| x.contains("records") || x.contains("images");
    dloader.get_all_available_file_list(file_filter, DownloadOption::Always).unwrap()
}
fn create_downloader() -> Downloader {

    let _ftp_conf = FtpConfig::init(config::Env::Production);
    let addr = "ftptds.shadowsoft.com:21";
    let user = "swg_eyemetric";
    let pass = "metric123swg99";

    //println!("{}", &ftp_conf.address);
    let path_vars = PathVars::new(config::Env::Production);

    Downloader::connect(addr, user, pass , path_vars).expect("Unable to get a good connection")
}

fn get_root_path(vars: &PathVars) -> PathBuf {
    PathBuf::from(&vars.vars["app_base_path"]).join(&vars.vars["archives_path"])
}

fn import_files() {

    let statelist: States = config::States::load().unwrap();
    //let statelist = statelist.iter().filter(|s| s.abbr != "TX" && s.abbr.chars().nth(0) > Some('H'));
    //     let statelist = statelist.iter().filter(|s| s.abbr != "HI" && s.abbr != "VA");
    //     let statelist = statelist.iter().filter(|s| s.abbr.chars().nth(0) > Some('H')); // && s.abbr.chars().nth(0) != Some('T'));
    //    let statelist = statelist.iter().filter(|s| s.abbr.chars().nth(0) == Some('T'));
    //let statelist = statelist.iter().filter(|s| s.abbr == "HI");
    let path_vars = PathVars::new(config::Env::Production);
    let  root_path = path_vars.archive_path();//util::get_root_path(&path_vars);

    let _prep_result = importer::prepare_import();

    let extract_opt = ExtractOptions::Default; //ImagesOnly;
    let extract_opt = ExtractOptions::ImagesOnly;
    let overwrite_files = true;
    for state in statelist {

    println!("=================================");
    println!("{}: {}", state.state, state.abbr);
    println!("=================================");

    let state_path = root_path.join(state.abbr.to_uppercase());
    println!("{:?}",&state_path);
    //let st_files = fs::read_dir(state_path).expect("A file but got us a directory");

    let st_files = fs::read_dir(&root_path).expect("A file but got us a directory");
    let st_files = st_files.filter(|fp| {
     let x = fp.as_ref().expect("a Dir Entry");
     x.file_name().to_str().unwrap()[..2] == state.abbr
    });
    importer::delete_old_photos(&state.abbr);
    for state_archive in st_files {

     println!("{:?}", state_archive);

     let archive = state_archive.unwrap();
     let mut extractor = Extractor::new(&path_vars);

     let extracted_files: Vec<ExtractedFile> = extractor.extract_archive(archive.path(), &extract_opt, overwrite_files).expect("A file but got a directory");

     for exfile in extracted_files {
         println!("importing file {:?}", archive.file_name());
         exfile.import().expect(&format!("Unable to complete file import {:?}", archive.file_name()));
         println!("=================================");
     }

}

importer::finalize_import(&state.abbr);

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
let  root_path =  path_vars.archive_path();

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
