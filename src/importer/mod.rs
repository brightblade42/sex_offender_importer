pub mod extracts;
pub mod img;

use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Write},
    path::{self, Path, PathBuf},
    string::ToString,
    error::Error
};

use ftp::types::FtpError::ConnectionError;
use csv::{self, Trim};
use rusqlite::{params, Connection, ToSql, NO_PARAMS};
use zip::{self, ZipArchive};
use regex::{self, bytes, Regex};
use serde::{Deserialize, Serialize};
use crate::util;
//submodules
use extracts::Csv;
use img::ImageArchive;

pub trait Import {
    type Reader;
    fn open_reader(&self, has_headers: bool) -> Result<Self::Reader , Box<dyn Error>>;
    fn import(&self) -> Result<(), Box<dyn Error>>;
    fn import_file_data(&self) -> Result<(), Box<dyn Error>>;
}


trait SqlHandler {
    type Reader;
    fn create_table_query(&self, reader: Option<&mut Self::Reader>, tname: &str) -> Result<String, Box<dyn Error>>;
    fn create_insert_query(&self,reader: &mut Self::Reader, tname: &str) -> Result<String, Box<dyn Error>>;

    fn create_default_index(&self, name: &str) -> String {

        format!( r#"CREATE  INDEX if not exists {}__index ON {} (
                 ID,
                 State
                 );"#, name, name )
    }

    fn drop_table(&self, conn: &Connection, table_name: &str) -> Result<usize, rusqlite::Error> {
        let r = conn.execute(&format!("DROP TABLE if exists {}", table_name), NO_PARAMS);
        r
    }
    fn execute(&self, conn: &Connection) -> Result<usize, rusqlite::Error>;
}



///ExtractedFile represents one of two possible types that are contained within
/// the zip archives downloaded from the remote server.
#[derive(Debug, Serialize, Deserialize)]
pub enum ExtractedFile {
    Csv(Csv),
    ImageArchive(ImageArchive)
}

impl Import for ExtractedFile {
    type Reader = String;

    fn open_reader(&self, has_headers: bool) -> Result<Self::Reader, Box<Error>> {
        unimplemented!()
    }

    fn import(&self) -> Result<(), Box<dyn Error>> {
        match self {

            ExtractedFile::Csv(csv) => {
                csv.import()
            },
            ExtractedFile::ImageArchive(img) => {
                img.import()
            }
        }
    }

    fn import_file_data(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

//=============================
//Module level functions
//================================
pub fn prepare_import() -> Result<(), Box<dyn Error>> {
    let conn = util::get_connection().expect("unable to connect to db");//Connection::open(sql_path)?;
    create_db(&conn).expect("something didn't create good.");
    conn.close();
    Ok(())
}

pub fn delete_old_photos(state: &str) -> Result<(), Box<dyn Error>> {
    let conn = util::get_connection()?;
    match conn.execute(
        &format!("DELETE FROM Photos where state='{}'", state),
        NO_PARAMS,
    ) {
        Ok(ex) => println!("deleted old photos for state: {}", state),
        Err(e) => println!("could not delete photos for state: {}. {}", state, e),
    }

    Ok(())
}

fn create_default_index(name: &str) -> String {
    format!(
        r#"CREATE  INDEX if not exists {}__index ON {} (
         ID,
         State
     );"#,
        name, name
    )
}

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

fn create_db(conn: &Connection) -> Result<(), Box<Error>> {
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

//this seems to do bupkis!
fn set_pragmas(conn: &Connection) {
    match conn.pragma_update(None, "journal_mode", &String::from("OFF")) {
        Ok(()) => println!("Updated pragma journal_mode: off"),
        Err(e) => println!("Could not update pragma journal_mode"),
    }
}

fn drop_table(conn: &Connection, table_name: &str) -> Result<usize, rusqlite::Error> {
    let r = conn.execute(&format!("DROP TABLE if exists {}", table_name), NO_PARAMS);
    //println!("Dropped Table: {}", table_name);
    r
}

fn create_blob_table_query() -> Result<String, Box<dyn Error>> {
    Ok(String::from(
        "CREATE TABLE if not exists Photos (id,name, size, data)",
    ))
}
