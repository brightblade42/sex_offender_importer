//! This module takes care of the import process which is comprised of reading csv files
//! cleaning data, ensuring sex offender db is setup properly and finally importing into sqlite
pub mod csv_importer;
pub mod img;

use std::{
    error::Error,
    fs,
    path::PathBuf
};
use rusqlite::Connection;
//use serde::{Deserialize, Serialize};
use crate::util::{ self, GenResult };
use crate::config::Config;

pub trait Import {
    type Reader;
    fn open_reader(&self, has_headers: bool) -> GenResult<Self::Reader>;
    fn import(&self) -> Result<(), Box<dyn Error>>;
    fn import_file_data(&self) -> Result<(), Box<dyn Error>>;
}

trait SqlHandler {
    type Reader;
    fn create_table_query(&self, reader: &mut Self::Reader, tname: &str) -> GenResult<String>;
    fn create_insert_query(&self,reader: &mut Self::Reader, tname: &str) -> GenResult<String>;

    fn create_default_index(&self, name: &str) -> String {

        format!( r#"CREATE  INDEX if not exists {}__index ON {} (
                 ID,
                 State
                 );"#, name, name )
    }

    //this could be a drop table or a delete all data
    fn drop_table(&self, conn: &Connection, table_name: &str) -> Result<usize, rusqlite::Error> {
        let r = conn.execute(&format!("DROP TABLE if exists {}", table_name), []);
        r
    }

    fn delete_data(&self, conn: &Connection, table_name: &str) -> Result<usize, rusqlite::Error> {
        conn.execute(&format!("Delete from {}", table_name), [])
    }

    fn execute(&self, conn: &Connection) -> Result<usize, rusqlite::Error>;
}


pub struct Importer {
    config:  Config,
    conn: Connection,
}

impl Importer {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
            conn: util::get_connection(config.offender_db.clone()).expect("a data connection")
        }
    }

    pub fn prepare_import(&self) -> GenResult<()> {
        self.create_db().expect("something didn't create good.");
        Ok(())
    }


    pub fn finalize_state_import(&self, state: &str) -> GenResult<()> {

        let mut pth = self.config.sql_path.to_path_buf();
        //let mut pth = PathBuf::from(util::SQL_FOLDER); //folder containing all the state level import queries
        pth.push(format!("{}_import.sql", state.to_lowercase()));

        if !pth.exists() {
            println!("The import file is missing: {}", &pth.display());
            return Ok(());
        }

        let final_import_query = fs::read_to_string(pth)?;
        let _res = self.conn.execute("BEGIN TRANSACTION", []);
        self.conn.execute(&format!("Delete from SexOffender where state='{}'", state), [])?;
        self.conn.execute(&final_import_query, [])?;
        let _res = self.conn.execute("END TRANSACTION", []);
        Ok(())
    }

    ///Performs final updates on the imported data.
    ///Formatting and cleanup jobs.
    pub fn finalize_import(&self) -> GenResult<()> {
        self.transform_photo_names()?;
        self.transform_date_of_births()?;
        Ok(())
    }


    fn transform_date_of_births(&self) -> GenResult<()> {
        self.execute_by_lines("formatDateOfBirth.sql")?;
        Ok(())
    }

    fn transform_photo_names(&self) -> GenResult<()> {
        self.execute_by_lines("formatPhotoNames.sql")?;
        Ok(())
    }

    ///load a files that contains a list of sql statements and execute each in turn.
    fn execute_by_lines(&self, sql_file_name: &str) -> GenResult<()>{

        let mut pth = self.config.sql_path.to_path_buf();
        //let mut pth = PathBuf::from(util::SQL_FOLDER); //folder containing all the state level import queries
        pth.push(sql_file_name);

        if !pth.exists() {
            println!("The sql file is missing: {}", &pth.display());
            return Ok(());
        }

        //let conn = util::get_connection(None)?;
        let _res = self.conn.execute("BEGIN Transaction", []);

        fs::read_to_string(pth)?.lines()
            .filter(|line| line.starts_with("update")) //only update queries
            .for_each(|line | {
                self.conn.execute(line, []).unwrap_or_else(|_| panic!("could not execute update {}", line));
            });

        let _res = self.conn.execute("END Transaction", []);
        Ok(())
    }

    ///returns a create index query string for some table_name
    fn create_default_index(&self, table_name: &str) -> String {
        format!(
            r#"CREATE  INDEX if not exists {}__index ON {} (
             ID,
             State
         );"#,
            table_name, table_name
        )
    }

    ///returns the sql query string that creates the main SexOffender table.
    fn create_main(&self) -> String {
        String::from(
            r#"CREATE TABLE IF NOT EXISTS SexOffender (
            id Integer,
            name,
            dateOfBirth,
            eyes,
            hair,
            height,
            weight,
            race,
            sex,
            state,
            aliases,
            addresses,
            offenses,
            scarsTattoos,
            photos
            )"#,
        )
    }

    ///ensures that all old photos are removed. Every time an import is run
    ///we delete before we insert. It keeps the engine clean.
    pub fn delete_old_photos(&self, state: &str) -> Result<usize, rusqlite::Error> {

        println!("deleting old photos");
        //let conn = util::get_connection(None).unwrap();
        self.conn.execute( &format!("DELETE FROM Photos where state='{}'", state), [], )
    }

    ///returns the sql query string that creates the main Photos table.
    fn create_photos(&self) -> String {
        String::from(
            r#"CREATE TABLE IF NOT EXISTS Photos (
            id INTEGER,
            name TEXT,
            size Integer,
            data Blob,
            state TEXT
        )"#,
        )
    }

    ///creates the necessary tables and indexes for the SexOffender database.
    fn create_db(&self) -> GenResult<()> {
        self.set_pragmas();
        self.conn.execute(self.create_main().as_str(), [])
            .expect("Unable to create main");
        self.conn.execute(self.create_photos().as_str(), [])
            .expect("unable to create photos");
        self.conn.execute(self.create_default_index("SexOffender").as_str(), [])
            .expect("unable to create main index");
        self.conn.execute(
            self.create_default_index("Photos").as_str(),
            [],
        )
            .expect("Unable to create photos index");
        Ok(())
    }

    //this seems to do bupkis! We're trying to set up some sqlite optimizations but
    //it seems to have no effect!
    fn set_pragmas(&self) {
        match self.conn.pragma_update(None, "synchronous", &String::from("OFF")) {
            Ok(()) => println!("Updated pragma synchronous: off"),
            Err(e) => println!("Could not update pragma synchronous {}", e),
        }
    }
}

