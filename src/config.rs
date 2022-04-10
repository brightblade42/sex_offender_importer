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
    //ftp
    pub ftp_base_path: &'static str, 
    pub ftp_sex_offender_path: &'static str,
    pub address: &'static str,
    pub pass: &'static str,
    pub name: &'static str,
    pub port: i32,
}

impl Config {

    pub fn new(root_path: PathBuf) -> Self {
        let extracts_path = root_path.join("extracts");
        let archives_path = root_path.join("archives");
        let offender_db = root_path.join("db/sexoffenders.sqlite");

        Self {
            root_path,
            archives_path,
            extracts_path,
            offender_db,
            //these should always be the same, except maybe for testing.
            ftp_base_path: "us",
            ftp_sex_offender_path: "/state/sex_offender", //utah/state/sex_offender
            address: "ftptds.shadowsoft.com",//host
            name: "swg_eyemetric", //user
            pass: "metric123swg99",
            port: 21
        }
     }

}

///a type that contains a states name and it's abbreviation.
#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub state: &'static str,
    pub abbr: &'static str,
}


//Contains the application directory paths based on
//the Env. Env determines (Production / Staging / Dev)
/*pub struct PathVars {
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
        PathBuf::from(&self.vars["app_base_path"]).join(&self.vars["archives_path"])
        //PathBuf::from(&self.vars["archives"])
    }

    ///returns the top level directory to the application
    pub fn root_path(&self) -> PathBuf {
        PathBuf::from(&self.vars["app_base_path"]) //.join(&self.vars["archives_path"])
    }

    ///returns the directory where we store the data we extract from the archive zip files.
    ///csv and image archives are store here for each state.
    pub fn extracts_path(&self) -> PathBuf {
        PathBuf::from(&self.vars["app_base_path"]).join(&self.vars["extracts"])
        //dfkPathBuf::from(&self.vars["extracts"])

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
*/


