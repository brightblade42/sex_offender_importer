//#[macro_use] extern crate lazy_static;
//! This module
use std::path::PathBuf;
use serde_derive::{Serialize, Deserialize};

#[derive( Debug, Clone)]
pub struct Config  {
    //local data paths
    pub root_path: PathBuf,
    pub archives_path: PathBuf,
    pub extracts_path: PathBuf,
    pub offender_db: PathBuf,
    pub sql_path: PathBuf
}

impl Config {

    pub fn new(root_path: PathBuf) -> Self {
        let extracts_path = root_path.join("data/extracts");
        let archives_path = root_path.join("data/archives");
        let offender_db = root_path.join("data/db/sexoffenders.sqlite");
        let sql_path = root_path.join("data/sql");
        Self {
            root_path,
            archives_path,
            extracts_path,
            offender_db,
            sql_path
        }
     }

}

///a type that contains a states name and it's abbreviation.
#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub state: &'static str,
    pub abbr: &'static str,
}

