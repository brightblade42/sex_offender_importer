pub mod extracts;
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
use extracts::Csv;
use img::ImageArchive;

pub trait Import {
    type Reader;
    fn open_reader(&self, has_headers: bool) -> GenResult<Self::Reader>;
    fn import(&self) -> Result<(), Box<dyn Error>>;
    fn import_file_data(&self) -> Result<(), Box<dyn Error>>;
}


trait SqlHandler {
    type Reader;
    fn create_table_query(&self, reader: Option<&mut Self::Reader>, tname: &str) -> GenResult<String>;
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



///ExtractedFile represents one of two possible types that are contained within
/// the zip archives downloaded from the remote server.
#[derive(Debug, Serialize, Deserialize)]
pub enum ExtractedFile {
    Csv(Csv),
    ImageArchive(ImageArchive)
}

impl Import for ExtractedFile {
    type Reader = String;

    fn open_reader(&self, _has_headers: bool) -> GenResult<Self::Reader> {
        unimplemented!()
    }

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

//=============================
//Module level functions
//================================
pub fn prepare_import() -> GenResult<()> {
    let conn = util::get_connection(None).expect("unable to connect to db");//Connection::open(sql_path)?;
    create_db(&conn).expect("something didn't create good.");
    Ok(())
}

pub fn finalize_import(state: &str) -> GenResult<()> {

    let mut pth = PathBuf::from(util::SQL_FOLDER);
    pth.push(format!("{}_import.sql", state.to_lowercase()));

    if !pth.exists() {
        println!("The import file is missing: {}", &pth.display());
        return Ok(());
    }

    let final_import_query = fs::read_to_string(pth)?;

    println!("=======================================" );
    println!("{}", &final_import_query);
    println!("=======================================" );

    let conn = util::get_connection(None)?;
    conn.execute(&format!("Delete from SexOffender where state='{}'", state), NO_PARAMS)?;
    conn.execute(&final_import_query, NO_PARAMS)?;

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

//this seems to do bupkis!
fn set_pragmas(conn: &Connection) {
    match conn.pragma_update(None, "journal_mode", &String::from("OFF")) {
        Ok(()) => println!("Updated pragma journal_mode: off"),
        Err(e) => println!("Could not update pragma journal_mode {}", e.description()),
    }
}
