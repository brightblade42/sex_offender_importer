//extern crate csv;
use csv;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path;

use rusqlite::{params, Connection, ToSql, NO_PARAMS};
use std::path::Path;
use std::string::ToString;
use super::types::ExtractedFile;
use zip;
use zip::ZipArchive;

static SQL_PATH: &'static str = "/home/d-rezzer/dev/sex_offender/archives/sexoffenders.sqlite";

fn open_csv_reader(file: File, delim: char) -> Result<csv::Reader<File>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(delim as u8)
        .from_reader(file);

    Ok(rdr)
}

fn create_default_index(name: &str) -> String {
    format!(
        r#"CREATE unique INDEX if not exists {}_idx ON {} (
         ID,
         State
     );"#,
        name, name
    )
}
fn create_default_non_unique_index(name: &str) -> String {
    format!(
        r#"CREATE INDEX if not exists {}_idx ON {} (
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
        age,
        dateOfBirth,
        state,
        aliases,
        offenses,
        addresses,
        photos,
        personalDetails )"#,
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
        create_default_non_unique_index("Photos").as_str(),
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
    println!("Dropped Table: {}", table_name);
    r
}
fn create_table_query(reader: &mut csv::Reader<File>, tname: &str) -> Result<String, Box<Error>> {
    let mut q = format!("CREATE TABLE if not exists {} (", tname);
    //add the header names as column names for out table.
    let mut create_table = reader
        .headers()
        .unwrap()
        .iter()
        .fold(q, |acc, head| format!("{} {}, ", acc, head));
    create_table.push_str("state )"); //add our extra state field and close the ()

    Ok(create_table)
}

fn create_blob_table_query() -> Result<String, Box<Error>> {
    Ok(String::from(
        "CREATE TABLE if not exists Photos (id,name, size, data)",
    ))
}

fn delete_old_photos(conn: &Connection, state: &str) {
    match conn.execute(
        &format!("DELETE FROM Photos where state='{}'", state),
        NO_PARAMS,
    ) {
        Ok(ex) => println!("deleted old photos for state: {}", state),
        Err(e) => println!("could not delete photos for state: {}. {}", state, e),
    }
}

fn create_insert_query(reader: &mut csv::Reader<File>, tname: &str) -> Result<String, Box<Error>> {
    //being constructing the insert statement.
    let mut q = format!("INSERT INTO {} ( ", tname);

    //add the headers as columns
    let mut insert_query = reader
        .headers()
        .unwrap()
        .iter()
        .fold(q, |acc, head| format!("{} {}, ", acc, head));

    //create the raw data tables. essentialy a copy of whatever crayzee
    //csv format we get.
    insert_query.push_str("state ) VALUES ("); //add last field and open VALUES.

    //add value parameter placeholders
    let header_count = reader.headers().unwrap().len();
    for i in 0..header_count {
        insert_query.push_str("?,");
    }

    insert_query.push_str("?)"); //our state parameter.

    Ok(insert_query)
}

use core::borrow::{Borrow, BorrowMut};

pub fn prepare_import() -> Result<(), Box<Error>> {
    let conn = Connection::open(SQL_PATH)?;

    create_db(&conn).expect("something didn't create good.");
    conn.close();
    Ok(())
}

//after the archived files have been extracted, we import them into
//a sqlite file
pub fn import_data(extracted_file: &ExtractedFile) -> Result<(), Box<dyn Error>> {
    use ExtractedFile::*;
    let conn = Connection::open(SQL_PATH)?;

    match extracted_file {
        Csv {
            path,
            state,
            delimiter,
        } => {
            //this should be filtered out, really.
            let dsp = path.display().to_string();
            if dsp.contains("screenshot") {
                //|| dsp.contains("photos") {
                return Ok(());
            }
            let file = File::open(path)?; //.unwrap();

            //how to decide on a delimiter
            let mut csv_reader = open_csv_reader(file, delimiter.to_owned())?;

            let table_name = String::from(path.file_stem().unwrap().to_str().unwrap());
            drop_table(&conn, &table_name)?;
            delete_old_photos(&conn, state.as_str());
            let mut table_query = create_table_query(&mut csv_reader, &table_name)?;
            conn.execute(&table_query, NO_PARAMS)?;

            let insert_query = create_insert_query(&mut csv_reader, &table_name)?;
            conn.execute("Begin Transaction;", NO_PARAMS);

            //insert a record (line of csv) into sqlite table.
            //we use as_bytes() because some data is not utf-8 compliant
            for result in csv_reader.byte_records() {
                match result {
                    Ok(record) => {
                        let mut rec = record.clone();
                        rec.push_field(state.as_bytes());
                        let res = conn.execute(&insert_query, &rec);
                    }
                    Err(e) => {
                        println!("Row data error: {}", e);
                    }
                }
            }
            conn.execute("COMMIT TRANSACTION;", NO_PARAMS);
        }

        ImageArchive { path, state } => {
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
                    name.find('.').expect("a damn index")
                };

                let photo_id = &name[0..idx];
                std::io::copy(&mut img_file, &mut blob);

                let r = conn
                    .execute(
                        "INSERT into photos (id,name, size, data,state) VALUES (?,?,?,?,?)",
                        params![photo_id, name, img_size, blob, state],
                    )
                    .expect("A damn image import");

                blob.clear();
            }

            conn.execute("COMMIT TRANSACTION;", NO_PARAMS);
        }
    };

    conn.close();
    Ok(())
}

//insert all the image files into a database table. Sounds wrong but it
//oh so right.
pub fn import_images(path: &Path) -> Result<(), Box<Error>> {
    let lpath = path::Path::new(path);
    //lpath.iter().for_each()
    Ok(())
}
