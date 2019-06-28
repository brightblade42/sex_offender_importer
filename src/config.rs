use std::io;
extern crate serde;
use serde_derive::{Serialize, Deserialize};

use rusqlite::{Connection, NO_PARAMS, params};
use serde_rusqlite;
use std::path::PathBuf;
use serde_rusqlite::{to_params_named, from_row, from_rows, from_rows_ref};

static CONFIG: &'static str = "/home/d-rezzer/code/eyemetric/sexoffenderimporter/sql/config.sqlite";
pub enum ConfigType {
    Test,
    Dev,
    Production
}
#[derive(Debug,Serialize, Deserialize)]
pub struct FtpConfig {
    pub address: String,
    pub user: String,
    pub pass: String,
    pub name: String,
    pub port: i32,

}

impl FtpConfig {
    fn conf(configType: &ConfigType) -> &'static str {
       match configType {
           ConfigType::Test => "local test",
           ConfigType::Dev => "public data",
           ConfigType::Production => "public data",
       }
    }
    pub fn init(configType: ConfigType) -> Self {
        
        let conn = Connection::open(CONFIG).expect("A good connection");
        let name = FtpConfig::conf(&configType);
        let mut stmt = conn.prepare("Select * from ftp where name = ?").expect("a statement "); //unwrap();
        let mut rows = stmt.query(params![name]).unwrap();
        let r = from_rows::<FtpConfig>(rows).last().expect("A row") ;

        r
    }
}


struct PathVar {
   name: String,
    value: PathBuf,
}


