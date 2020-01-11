use std::{
    path::{PathBuf},
    collections::HashMap,
    error::Error
};

use serde_derive::{Serialize, Deserialize};
use rusqlite::{Connection, NO_PARAMS, params};
use serde_rusqlite::{self,  from_rows };
use std::path::Path;

static CONFIG: &'static str = "/opt/eyemetric/sex_offender/app/config.sqlite";


///Represents development state of the Application
///Useful for loading different configurations for various development scenarios
#[derive(Debug, Serialize, Deserialize)]
pub enum Env {
    Test,
    Dev,
    Production,
}

///use Config to load a set of configuration variables based on an Environment (env).
pub trait Config {
    fn load(env: &Env) -> ConfigResult;//Result<HashMap<String,String>, Box<std::error::Error>>;
}

///A set of key value string pairs representing configuration settings.
pub type ConfigResult = Result<HashMap<String, String>, Box<dyn std::error::Error>>;
///a Vec of State objects. States as in US states not some program state.
pub type States = Vec<State>;

///TODO: figure out why we didn't use Config
pub trait LoadData {
    fn load() -> Result<States, Box<dyn Error>>;
}

impl LoadData for States {

    fn load() -> Result<States, Box<dyn Error>> {
        let conn = Connection::open(CONFIG).expect("Unable to open data connection");
        let mut stmt = conn.prepare("Select * from states").expect("Unable to get states data");
        let rows = stmt.query(NO_PARAMS)?;
        let r = from_rows::<State>(rows).collect();
        Ok(r)
    }
}

///a type that contains a states name and it's abbreviation.
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
        let rows = stmt.query(params![name]).unwrap();
        let r = from_rows::<FtpConfig>(rows).last().expect("A row");

        r
    }
}

///Contains the application directory paths based on
///the Env. Env determines (Production / Staging / Dev)
pub struct PathVars {
    pub env: Env,
    pub vars: HashMap<String, String>,
}

impl PathVars {
    ///returns PathVars value which contains the data directories for an environment.
    pub fn new(env: Env) -> Self {
        let vars = PathVars::load(&env).expect("Unable to create PathVars hash map");

        PathVars {
            env,
            vars,
        }
    }

    ///return path to archive folder. This is where we store the zip files downloaded
    ///from the server.
    pub fn archive_path(&self) -> PathBuf {
        //PathBuf::from(&self.vars["app_base_path"]).join(&self.vars["archives_path"])
        PathBuf::from(&self.vars["archives"])
    }

    ///returns the top level directory to the application
    pub fn root_path(&self) -> PathBuf {
        PathBuf::from(&self.vars["app_base_path"]) //.join(&self.vars["archives_path"])
    }

    ///returns the directory where we store the data we extract from the archive zip files.
    ///csv and image archives are store here for each state.
    pub fn extracts_path(&self) -> PathBuf {
        PathBuf::from(&self.vars["extracts"])

    }
    ///returns the full path to an archive (zip) file
    pub fn archive_file_path(&self, file_name: &str) -> PathBuf {
        PathBuf::from(self.archive_path().display().to_string()).join(file_name)
    }

    ///return the path to the sexoffender.sqlite file.
    pub fn sql_path(&self) -> PathBuf {
         PathBuf::from(&self.vars["app_base_path"]).join(&self.vars["sex_offender_db"])
    }

}

impl Config for PathVars {
    ///takes an env value and returns ConfigResult which is the set of key-value
    ///configuration variables.
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



