#![allow(dead_code)]

use structopt::StructOpt;
use indicatif::{ProgressStyle, ProgressBar};
use std::fs;
use sex_offender::{
    importer::{ Import, Importer },
    extractors::{Extractor,  ExtractOptions},
    config::{Config, State },
};

/*
#[derive(Debug, StructOpt)]
#[structopt(name= "sx-imp", about = "cli tool for importing sex offender data",
author="ryan lee martin")]
enum SXOCli {

    #[structopt(name="import")]
    Import {
        #[structopt(long,short)]
        ///pass -a to begin import for all extracted files
        ///
        ///The import process will delete any existing state data in favor of the update.
        ///imports are all or nothing at the state level
        all: bool,
        #[structopt(long,short)]
        ///pass -i to import just the new images
        img: bool,
        #[structopt(long,short)]
        ///pass -c to import just the csv data.
        csv: bool,
        #[structopt(short="s", long, help="add a comma separated list of states by abbreviation to filter the imports")]
        state: String
    },
}
*/
#[derive(StructOpt, Debug)]
#[structopt(name= "sx-imp", about = "cli tool for importing sex offender data")]
struct CLI {
    #[structopt(long,short)]
    state: Option<Vec<String>>
}


fn get_states() -> Vec<State> {
    vec![ 
        State { state: "Alabama", abbr: "AL"}, State { state: "Alaska", abbr: "AK"},
        State { state: "Arizona", abbr: "AZ"}, State { state: "Arkansas", abbr: "AR"},
        State { state: "California", abbr: "CA"}, State { state: "Colorado", abbr: "CO"},
        State { state: "Connecticut", abbr: "CT"}, State { state: "Delaware", abbr: "DE"},
        State { state: "Florida", abbr: "FL"}, State { state: "Georgia", abbr: "GA"},
        State { state: "Hawaii", abbr: "HI"}, State { state: "Idaho", abbr: "ID"},
        State { state: "Illinois", abbr: "IL"}, State { state: "Indiana", abbr: "IN"},
        State { state: "Iowa", abbr: "IA"}, State { state: "Kansas", abbr: "KS"},
        State { state: "Kentucky", abbr: "KY"}, State { state: "Louisiana", abbr: "LA"},
        State { state: "Maine", abbr: "ME"}, State { state: "Maryland", abbr: "MD"},
        State { state: "Massachusetts", abbr: "MA"}, State { state: "Michigan", abbr: "MI"},
        State { state: "Minnesota", abbr: "MN"}, State { state: "Mississippi", abbr: "MS"},
        State { state: "Missouri", abbr: "MO"}, State { state: "Montana", abbr: "MT"},
        State { state: "Nebraska", abbr: "NE"}, State { state: "Nevada", abbr: "NV"},
        State { state: "New_Hampshire", abbr: "NH"}, State { state: "New_Jersey", abbr: "NJ"},
        State { state: "New_Mexico", abbr: "NM"}, State { state: "New_York", abbr: "NY"},
        State { state: "North_Carolina", abbr: "NC"}, State { state: "North_Dakota", abbr: "ND"},
        State { state: "Ohio", abbr: "OH"}, State { state: "Oklahoma", abbr: "OK"},
        State { state: "Oregon", abbr: "OR"}, State { state: "Pennsylvania", abbr: "PA"},
        State { state: "Rhode_Island", abbr: "RI"}, State { state: "South_Carolina", abbr: "SC"},
        State { state: "South_Dakota", abbr: "SD"}, State { state: "Tennessee", abbr: "TN"},
        State { state: "Texas", abbr: "TX"}, State { state: "Utah", abbr: "UT"},
        State { state: "Vermont", abbr: "VT"}, State { state: "Virginia", abbr: "VA"},
        State { state: "Washington", abbr: "WA"}, State { state: "West_Virginia", abbr: "WV"},
        State { state: "Wisconsin", abbr: "WI"}, State { state: "Wyoming", abbr: "WY"},
    ]

}

fn main()  {

    //root path could come from env
    let config = Config::new(std::env::current_dir().unwrap());
    //let opt = SXOCli::from_args();
    let opt = CLI::from_args();


    println!("something");
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

    if opt.state.is_some() {
            println!("opt {:#?}", opt.state);
            pb.set_message(format!("Importing extracted files for : {:?}", opt.state.as_ref().unwrap()));
            import_files(&config, opt.state);
            pb.finish_with_message("Import has completed");
    } else {
        println!("sup dude");
    }
    
    /*
    match opt {

        SXOCli::Import { all, ..} if all  => {
            pb.set_message("Importing all extracted files...");
            import_files(&config);
            pb.finish_with_message("Import has completed");
        },
        SXOCli::Import { state, .. } => {
            println!("you want to import : {:?}", state);
        }

        //_ => { println!("not ready for that one chief");}
    }*/
}

///start the importing
fn import_files(config: &Config, states: Option<Vec<String>>) {

    let state_v = get_states();
    let statelist = match states {
        Some(states) => {
           state_v.into_iter().filter(|s| { states.contains(&s.abbr.to_owned()) }).collect()
        },
        None => state_v
        
    };
    if statelist.is_empty() {
        println!("There were no matching states. check your state abbreviations!");
        return;
    }
    //TODO: get state list from cmd line
    //let statelist = state_v.iter().filter(|s| s.abbr == "NY"); 
    let archive_path = &config.archives_path; 
    let importer = Importer::new(config);
    let _prep_result = importer.prepare_import(); 
    let extract_opt = ExtractOptions::Default; 
    let overwrite_files = true;

    for state in statelist {

        println!("=================================");
        println!("{}: {}", state.state, state.abbr);
        println!("=================================");

        let state_archive_path = archive_path.join(state.abbr.to_uppercase());
        println!("Achive Path:: {:?}",&state_archive_path);
        fs::create_dir_all(&state_archive_path).expect("ability to create a dir");
        let st_files = fs::read_dir(&state_archive_path).expect("A file but got us a directory");
        let st_files = st_files.filter(|fp| {
             let x = fp.as_ref().expect("a Dir Entry");
             &x.file_name().to_str().unwrap()[..2] == state.abbr
        });
        
        let _res = importer.delete_old_photos(state.abbr); //from main photo table

        for state_archive in st_files {

             println!("state archive: {:?}", state_archive);

             let archive = state_archive.unwrap();
             println! ("{:?}", archive.file_name());
             let mut extractor = Extractor::new(config);

             let extracted_files = extractor.extract_archive(archive.path(), &extract_opt, overwrite_files)
                 .expect("A file but got a directory");

            //each archive contains 1 or more files
             for exfile in extracted_files {
                 println!("importing file {:?}", archive.file_name());
                 exfile.import().unwrap_or_else(|_| panic!("Unable to complete file import {:?}", archive.file_name()));
                 println!("=================================");
             }
        }

        let _res = importer.finalize_state_import(state.abbr);
        println!("=================================");
    }

    let _res = importer.finalize_import();
    println!("The import has completed");

}


