//! This module takes care of the import process which is comprised of reading csv files
//! cleaning data, ensuring sex offender db is setup properly and finally importing into sqlite
pub mod csv_importer;
pub mod img;

use std::{
    error::Error,
    fs,
    path::{PathBuf}
};
use rusqlite::{Connection, NO_PARAMS};
use serde::{Deserialize, Serialize};
use crate::util::{
    self,
    GenResult
};
//submodules
use csv_importer::Csv;
use img::ImageArchive;


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
        let r = conn.execute(&format!("DROP TABLE if exists {}", table_name), NO_PARAMS);
        r
    }

    fn delete_data(&self, conn: &Connection, table_name: &str) -> Result<usize, rusqlite::Error> {
        conn.execute(&format!("Delete from {}", table_name), NO_PARAMS)
    }

    fn execute(&self, conn: &Connection) -> Result<usize, rusqlite::Error>;
}



///ExtractedFile represents either a CSV file or  embedded Zip files (image files are stored as zip files
/// within an outer zip file) that are contained within
/// the top-level zip archives downloaded from the `public data` remote server.
#[derive(Debug, Serialize, Deserialize)]
pub enum ExtractedFile {
    Csv(Csv),
    ImageArchive(ImageArchive)
}

///we implement Import trait on Extracted file to forward import calls
///to its variants.
impl Import for ExtractedFile {
    type Reader = String;

    fn open_reader(&self, _has_headers: bool) -> GenResult<Self::Reader> {
        unimplemented!()
    }

    ///forwards the import call to the specific import methods on CSV or ImageArchive,
    ///depending on which is matched.
    fn import(&self) -> GenResult<()> {
        match self {

            ExtractedFile::Csv(csv) => {
                csv.import()
            },
            ExtractedFile::ImageArchive(img) => {
                img.import()
            }
        }
    }

    fn import_file_data(&self) -> GenResult<()> {
        Ok(())
    }
}

///ensures that the database schema is created before we try to import any data into the db.
pub fn prepare_import() -> GenResult<()> {
    let conn = util::get_connection(None).expect("unable to connect to db");
    create_db(&conn).expect("something didn't create good.");

    Ok(())
}

///takes a state abbreviation and locates its corresponding ST_import.sql file.
///An import sql file exists for each state which is responsible for importing
///data from each individual state table into a common SexOffender table.
pub fn finalize_import(state: &str) -> GenResult<()> {

    let mut pth = PathBuf::from(util::SQL_FOLDER); //folder containing all the state level import queries
    pth.push(format!("{}_import.sql", state.to_lowercase()));

    if !pth.exists() {
        println!("The import file is missing: {}", &pth.display());
        return Ok(());
    }

    let final_import_query = fs::read_to_string(pth)?;
    let conn = util::get_connection(None)?;
    conn.execute("BEGIN TRANSACTION", NO_PARAMS);
    conn.execute(&format!("Delete from SexOffender where state='{}'", state), NO_PARAMS)?;
    conn.execute(&final_import_query, NO_PARAMS)?;
    conn.execute("END TRANSACTION", NO_PARAMS);
    Ok(())
}

///many of the dates are not in valid but inconsistent format.
///This function loads a sql file an converts all known formats into
///one unified format of MM/DD/YYYY
pub fn transform_date_of_births() -> GenResult<()> {

    let mut pth = PathBuf::from(util::SQL_FOLDER); //folder containing all the state level import queries
    pth.push("dataOfBirthConversion.sql");

    if !pth.exists() {
        println!("The import file is missing: {}", &pth.display());
        return Ok(());
    }

    let date_of_births_conversion = fs::read_to_string(pth)?;
    let conn = util::get_connection(None)?;
    conn.execute(&date_of_births_conversion, NO_PARAMS)?;

    Ok(())
}
///returns a create index query string for some table_name
fn create_default_index(table_name: &str) -> String {
    format!(
        r#"CREATE  INDEX if not exists {}__index ON {} (
         ID,
         State
     );"#,
        table_name, table_name
    )
}

///returns the sql query string that creates the main SexOffender table.
fn create_main() -> String {
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
pub fn delete_old_photos(state: &str) -> Result<usize, rusqlite::Error> {

    println!("Are you my mommy?");
    let conn = util::get_connection(None).unwrap();
    conn.execute( &format!("DELETE FROM Photos where state='{}'", state), NO_PARAMS, )
}

///returns the sql query string that creates the main Photos table.
fn create_photos() -> String {
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
fn create_db(conn: &Connection) -> GenResult<()> {
    set_pragmas(conn);
    conn.execute(create_main().as_str(), NO_PARAMS)
        .expect("Unable to create main");
    conn.execute(create_photos().as_str(), NO_PARAMS)
        .expect("unable to create photos");
    conn.execute(create_default_index("SexOffender").as_str(), NO_PARAMS)
        .expect("unable to create main index");
    conn.execute(
        create_default_index("Photos").as_str(),
        NO_PARAMS,
    )
        .expect("Unable to create photos index");
    Ok(())
}

//this seems to do bupkis! We're trying to set up some sqlite optimizations but
//it seems to have no effect!
fn set_pragmas(conn: &Connection) {
/*    match conn.pragma_update(None, "journal_mode", &String::from("OFF")) {
        Ok(()) => println!("Updated pragma journal_mode: off"),
        Err(e) => println!("Could not update pragma journal_mode {}", e.description()),
    }
*/
    match conn.pragma_update(None, "synchronous", &String::from("OFF")) {
        Ok(()) => println!("Updated pragma synchronous: off"),
        Err(e) => println!("Could not update pragma synchronous {}", e.description()),
    }
}


