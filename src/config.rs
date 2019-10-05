use std::{
    io,
    path::{PathBuf, Iter},
    iter::FromIterator,
    env,
    collections::HashMap,
    hash::Hash,
    error::Error
};

use serde_derive::{Serialize, Deserialize};
use rusqlite::{Connection, NO_PARAMS, params};
use serde_rusqlite::{self, to_params_named, from_row, from_rows, from_rows_ref};

static CONFIG: &'static str = "/opt/eyemetric/sex_offender/config.sqlite";


#[derive(Debug, Serialize, Deserialize)]
pub enum Env {
    Test,
    Dev,
    Production,
}


pub trait Config {
    fn load(env: &Env) -> ConfigResult;//Result<HashMap<String,String>, Box<std::error::Error>>;
}

pub type States = Vec<State>;

pub trait LoadData {
    fn load() -> Result<States, Box<dyn Error>>;
}

impl LoadData for States {
    fn load() -> Result<States, Box<dyn Error>> {
        let conn = Connection::open(CONFIG).expect("Unable to open data connection");
        let mut stmt = conn.prepare("Select * from states").expect("Unable to get states data");
        let mut rows = stmt.query(NO_PARAMS)?;
        let r = from_rows::<State>(rows).collect();
        Ok(r)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub state: String,
    pub abbr: String,
}


#[derive(Debug, Serialize, Deserialize)]
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
        let r = from_rows::<FtpConfig>(rows).last().expect("A row");

        r
    }
}

pub type ConfigResult = Result<HashMap<String, String>, Box<dyn std::error::Error>>;

pub struct PathVars {
    pub env: Env,
    pub vars: HashMap<String, String>,
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
        let mut stmt = conn.prepare("Select * from paths where env = ?")?;
        let mut hashmp: HashMap<String, String> = HashMap::new();
        let en = match env {
            Env::Test => "Test",
            Env::Dev => "Dev",
            Env::Production => "Production",
        };
        let mut rows = stmt.query(&[en])?;

        while let Some(row) = rows.next()? {
            hashmp.insert(row.get(0)?, row.get(1)?);
        }


        Ok(hashmp)
    }
}



