//extern crate csv;
use csv::{self, Trim};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path;

use rusqlite::{params, Connection, ToSql, NO_PARAMS};
use std::path::{Path, PathBuf};
use std::string::ToString;
use zip;
use zip::ZipArchive;
use regex::{self, bytes, Regex};
use serde::{Deserialize, Serialize};
pub mod extracts;
pub mod img;
mod util;


use extracts::Csv;
use img::ImageArchive;

pub trait Import {
    type Reader;
    fn open_reader(&self, has_headers: bool) -> Result<Self::Reader , Box<dyn Error>>;
    fn import(&self) -> Result<(), Box<dyn Error>>;
    fn import_file_data(&self, conn: &Connection) -> Result<(), Box<dyn Error>>;
}


trait SqlHandler {
    type Reader;
    fn create_table_query(&self, reader: Option<&mut Self::Reader>, tname: &str) -> Result<String, Box<dyn Error>>;
    fn create_insert_query(&self,reader: &mut Self::Reader, tname: &str) -> Result<String, Box<dyn Error>>;
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

    //not using
    fn import(&self) -> Result<(), Box<dyn Error>> {
        match self {

            ExtractedFile::Csv(csv) => {
                csv.import()
            },
            ExtractedFile::ImageArchive(img) => {
                img.import()
            }
            // _ => Ok(())
        }
    }

    fn import_file_data(&self, conn: &Connection) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

//=============================
//Module level functions
//================================
pub fn prepare_import(sql_path: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(sql_path)?;
    create_db(&conn).expect("something didn't create good.");
    conn.close();
    Ok(())
}

//after the archived files have been extracted, we import them into
//a sqlite file
pub fn import_data(extracted_file: &ExtractedFile, sql_path: &str) -> Result<(), Box<dyn Error>> {
    use ExtractedFile::*;
    let conn = Connection::open(sql_path)?;

    match extracted_file {
        Csv(csv) => {
            import_csv_files(&conn, &csv.path, &csv.state, &csv.delimiter);
        }
        ImageArchive(img)  => {
            import_images(&conn, &img.path, &img.state);
        }
    };

    conn.close();
    Ok(())
}

pub fn delete_old_photos(state: &str, sql_path: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(sql_path)?;
    match conn.execute(
        &format!("DELETE FROM Photos where state='{}'", state),
        NO_PARAMS,
    ) {
        Ok(ex) => println!("deleted old photos for state: {}", state),
        Err(e) => println!("could not delete photos for state: {}. {}", state, e),
    }

    Ok(())
}
fn open_csv_reader(file: File, delim: char, has_headers: bool) -> Result<csv::Reader<File>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(delim as u8)
        .trim(csv::Trim::All)
        .has_headers(has_headers)
      //  .flexible(true)
        .from_reader(file);

    Ok(rdr)
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

fn convert_state_field(field: &str) -> &str {
    if field == "State" || field == "state" {
        "Addr_State"
    } else {
        field
    }
}

fn convert_space_in_field(field: &str) -> String {
    if field.trim().contains(" ") {
        field.replace(" ", "_")
    } else {
        field.to_owned()
    }
}

fn create_table_query(reader: &mut csv::Reader<File>, tname: &str) -> Result<String, Box<dyn Error>> {
    let mut q = format!("CREATE TABLE if not exists {} (", tname);
    //add the header names as column names for out table.
    let mut create_table = reader
        .headers()
        .unwrap()
        .iter()
        .map(convert_state_field)
        .map(convert_space_in_field)
        .fold(q, |acc, head| {
            format!("{} {},", acc, head.replace("/", ""))
        });

    create_table.push_str("state )"); //add our extra state field and close the ()

    Ok(create_table)
}

fn create_blob_table_query() -> Result<String, Box<dyn Error>> {
    Ok(String::from(
        "CREATE TABLE if not exists Photos (id,name, size, data)",
    ))
}


fn create_insert_query(reader: &mut csv::Reader<File>, tname: &str) -> Result<String, Box<dyn Error>> {
    //being constructing the insert statement.
    let mut q = format!("INSERT INTO {} ( ", tname);

    //add the headers as columns
    let mut insert_query = reader
        .headers()
        .unwrap()
        .iter()
        .map(convert_state_field)
        .map(convert_space_in_field)
        .fold(q, |acc, head| format!("{} {},", acc, head.replace("/", "")));

    insert_query.push_str("state ) VALUES ("); //add our extra state field and close the ()

    //add value parameter placeholders
    //TODO: this seems like it could be better.
    let header_count = reader.headers().unwrap().len();

    for i in 0..header_count {
        insert_query.push_str("?,");
    }

    insert_query.push_str("?)"); //our state parameter.
    Ok(insert_query)
}





fn format_date(dateStr: &str) -> &str {
      let reg =  Regex::new(r"\d{2}/\d{2}/d{4}").unwrap();
    if reg.is_match(dateStr) {
        "coool"
    } else {
        dateStr
    }
    //regex::bytes::
}

fn import_csv_files(conn: &Connection, path: &PathBuf, state: &str, delimiter: &char) -> Result<(), Box<dyn Error>> {

    //this should be filtered out, really.
    let dsp = path.display().to_string();
    //TODO: filter this out from elsewhere methinks.
    if dsp.contains("screenshot") {
        println!("skipping screenshot file. It is useless");
        return Ok(());
    }
    let file = File::open(path)?; //.unwrap();

   // let has_headers = if state == "TX" { false } else { true };
    //how to decide on a delimiter
    let mut csv_reader = open_csv_reader(file, delimiter.to_owned(), true)?;

    let table_name = String::from(path.file_stem().unwrap().to_str().unwrap());
    println!("=============================");
    println!("Dropped: {}", &table_name);
    drop_table(&conn, &table_name)?;

    let mut table_query = create_table_query(&mut csv_reader, &table_name)?;

    println!("Creating {}", &table_name);
    println!("=============================");
    println!("{}", &table_query);
    println!("=============================");
    conn.execute(&table_query, NO_PARAMS)?;

    //create default index
    let defindex = create_default_index(&table_name);
    conn.execute(&defindex, NO_PARAMS);


    let insert_query = create_insert_query(&mut csv_reader, &table_name)?;

    println!("=============================");
    println!("{}", &insert_query);

    println!("=============================");

    conn.execute("Begin Transaction;", NO_PARAMS);
    let mut rec_vals: Vec<String> = Vec::new();
    for result in csv_reader.byte_records() {
        match result {
            Ok(record) => {
                let mut rec = record.clone();

               for rr in rec.iter() {
                   let ascii_string = util::to_ascii_string(&rr);
                   rec_vals.push(ascii_string.parse().unwrap());
                }

                if table_name == "OFF_CODE_SOR" {
                    println!("{}",rec.len());
                }

                rec_vals.push(String::from(state));

                let res = conn.execute(&insert_query, &rec_vals).expect("Good stuff");
                rec_vals.clear();
            }
            Err(e) => {
                println!("Row data error: {}", e);
            }
        }
    }
    conn.execute("COMMIT TRANSACTION;", NO_PARAMS);
    Ok(())
}


fn import_images(conn: &Connection, path: &PathBuf, state: &str) -> Result<(), Box<dyn Error>> {
    let blob_table = create_blob_table_query();
    conn.execute(&blob_table.unwrap(), NO_PARAMS);

    //1. we've got an archive of images. we don't want to write them
    //to disk, we want to store them as blobs in sqlite.

    //iterate images,
    //validate,
    //write to Vec<u8>
    //write bytes to db.
    let mut blob: Vec<u8> = vec![];

    let file = BufReader::new(File::open(path).unwrap());
    let mut archive = zip::ZipArchive::new(file)?;

    conn.execute("BEGIN TRANSACTION;", NO_PARAMS);

    for i in 0..archive.len() {
        let mut img_file = archive.by_index(i)?;
        let img_name = img_file.sanitized_name();
        let img_size = img_file.size() as u32;
        let name = img_name.display().to_string();
        let idx = if let Some(p) = name.find('_') {
            p
        } else {
            //name.find('.').expect("a damn index")
            name.len()
        };

        if name.contains("table") { //not an image
            continue;
        }
        let photo_id = &name[0..idx];
        std::io::copy(&mut img_file, &mut blob);

        //println!("image: {} {} {} {}", photo_id, name, img_size, state);
        let r = conn
            .execute(
                "INSERT into Photos (id,name, size, data,state) VALUES (?,?,?,?,?)",
                params![photo_id, name, img_size, blob, state],
            )
            .expect("A damn image import");

        blob.clear();
    }

    conn.execute("COMMIT TRANSACTION;", NO_PARAMS);

    Ok(())
}
//insert all the image files into a database table. Sounds wrong but it
//oh so right.
/*pub fn import_images(path: &Path) -> Result<(), Box<Error>> {
    let lpath = path::Path::new(path);
    //lpath.iter().for_each()
    Ok(())
}*/
