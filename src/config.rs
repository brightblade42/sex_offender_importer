use std::io;
extern crate serde;
use serde_derive::{Serialize, Deserialize};

use rusqlite::{Connection, NO_PARAMS, params};
use serde_rusqlite;
use std::path::{PathBuf, Iter};
use serde_rusqlite::{to_params_named, from_row, from_rows, from_rows_ref};
use std::iter::FromIterator;
use std::env;
use std::collections::HashMap;
use crate::downloader::SexOffenderImportError::ConnectionError;
use std::hash::Hash;

static CONFIG: &'static str = "/opt/eyemetric/sex_offender/config.sqlite";


#[derive(Debug,Serialize, Deserialize)]
pub enum Env {
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
    fn conf(env: &Env) -> &'static str {
       match env {
           Env::Test => "local test",
           Env::Dev => "public data",
           Env::Production => "public data",
       }
    }
    pub fn init(env: Env) -> Self {
        
        let conn = Connection::open(CONFIG).expect("A good connection");
        let name = FtpConfig::conf(&env);
        let mut stmt = conn.prepare("Select * from ftp where name = ?").expect("a statement "); //unwrap();
        let mut rows = stmt.query(params![name]).unwrap();
        let r = from_rows::<FtpConfig>(rows).last().expect("A row") ;

        r
    }
}

pub type ConfigResult = Result<HashMap<String,String>, Box<dyn std::error::Error>>;

pub trait Config {
    fn load(env: &Env) -> ConfigResult;//Result<HashMap<String,String>, Box<std::error::Error>>;
}
pub struct PathVars {
   pub env: Env,
   pub vars: HashMap<String, String>
}

impl PathVars {
    pub fn new(env: Env) -> Self {
        let vars = PathVars::load(&env).expect("Unable to create PathVars hash map");

        PathVars {
            env,
            vars,
        }
    }

}

impl Config for PathVars {

    fn load(env: &Env) -> ConfigResult //Result<HashMap<String,String>, Box<std::error::Error>>
    {
        let conn = Connection::open(CONFIG)?;
        let mut stmt = conn.prepare("Select * from kpaths where env = ?")?;
        let mut hashmp: HashMap<String, String> = HashMap::new();
        let en = match env {

            Env::Test => "Test",
            Env::Dev => "Dev",
            Env::Production => "Production",
        } ;
        let mut rows = stmt.query(&[en])?;

        while let Some(row) = rows.next()? {
            hashmp.insert(row.get(0)?, row.get(1)?);
        }


        Ok(hashmp)
    }


}



