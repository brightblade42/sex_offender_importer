
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
use sex_offender::util::GenResult;
use sex_offender::downloader::records::FileInfo;

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
        SXOCli::List { new, ..} => {

            let new_list = get_new_list(&mut downloader);

            let mut table = Table::new();
            table.add_row(row!["FILES READY FOR DOWNLOAD", &format!("total: {}",new_list.len())]);
            table.add_row(row!["path", "file name", "last mod"]);
            for item in new_list {

                let f = item.unwrap();
                let path = f.remote_path().display().to_string();
                let name = f.name();
//                println!("{:?}", item.unwrap().name());
                table.add_row(row![path,name,"time is a construct" ]);
            }

            table.printstd();

/*
            println!("you want the new stuff huh bro");
            let table = table!(["ABC","DEFG","HIJKLMNOP"],
            ["foobar","bar","foo"],
            ["monkey","puzzle","sort"]);

            let table1 = table!(["ABC", "DEFG", "HIJKLMN"],
                    ["foobar", "bar", "foo"],
                    ["foobar2", "bar2", "foo2"]);

            let table2 = table!(["Title 1", "Title 2"],
                    ["This is\na multiline\ncell", "foo"],
                    ["Yo dawg ;) You can even\nprint tables\ninto tables", table1],
                    ["Like far out man,\n Far fucking out!", table]);

            table2.printstd();

            ptable!([bFg->"foobar", BriH2->"bar", "foo"]);
*/
        },
        SXOCli::List { all, ..} => {
            println!("you want the ALL THE stuff huh bro");
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
                println!("You wanna get that sweet new shit huh bro!?") ;
            } else if let Some(s) = state {
               println!("Just this state bro? {}", s);
            }
        },

        _ => { println!("not ready for that one chief");}
    }

/*   match opt {
        SexOffenderCli { list, ..} if list == "cool" => {
            println!("Damn right you are");
            let record_filter = |x: &String| x.contains("records") || x.contains("images");

            downloader.rebuild_log_from_archives(); */
            //let record_filter = |x: &String| x.contains("records");

           /* let file_list = downloader.get_all_available_file_list(record_filter, DownloadOption::Always);

            for r in file_list {
                println!("{:?}", r);
            }
*           */
/*        },
       _ => {
           println!("{:#?}", opt);
       }
    }*/

//    download_files();
//    import_files();

    //println!("{:#?}", opt);
}
fn get_new_list(dloader: &mut Downloader) -> Vec<GenResult<FileInfo>>{

    let record_filter = |x: &String| x.contains("records");
    dloader.get_newest_update_list()

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

fn get_updated_file_list() {

}
fn download_files()  {

//maybe just download the enchilada?
//let avail; //entire list
//let avail_updates; //new than what we already have

let _ftp_conf = FtpConfig::init(config::Env::Production);
let addr = "ftptds.shadowsoft.com:21";
let user = "swg_eyemetric";
let pass = "metric123swg99";

//println!("{}", &ftp_conf.address);
let path_vars = PathVars::new(config::Env::Production);


let mut downloader = Downloader::connect(addr, user, pass ,
                                  path_vars).expect("Unable to get a good connection");

let record_filter = |x: &String| x.contains("records") || x.contains("images");

//let record_filter = |x: &String| x.contains("records");
let file_list = downloader.get_all_available_file_list(record_filter, DownloadOption::Always);
let file_list = downloader.get_newest_update_list();

// println!("{:?}", file_list);



let sex_offender_archives =  file_list.iter()
.map(|fi| {
 let m = downloader.download_file(&fi.as_ref().unwrap());
 println!("downloaded: {}", fi.as_ref().unwrap().name());
 m
});

for arch in sex_offender_archives {

if let Ok(sx) = arch {
 println!("all good {}", sx.path.display());
 //do something
} else {
 println!("not good at all.");
 //err, bad download, add to error queue?
}
}


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
