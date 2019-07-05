//extern crate csv;
use std::path;
use std::io::{BufReader, BufWriter, Read, Write};
use std::error::Error;
use csv;
use std::fs;
use std::fs::File;

use rusqlite::{Connection, params, NO_PARAMS, ToSql};
use std::path::Path;
use std::string::ToString;

use zip;
use zip::ZipArchive;
static SQL_PATH: &'static str = "/home/d-rezzer/dev/sex_offender/archives/sexoffenders.sqlite";

fn open_csv_reader(file: File) -> Result<csv::Reader<File>, Box<Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .from_reader(file);

    Ok(rdr)
}

fn create_default_index(name: &str) -> String {
    format!(
        r#"CREATE unique INDEX if not exists {}_idx ON {} (
         ID,
         State
     );"#, name, name)
}
fn create_default_non_unique_index(name: &str) -> String {
    format!(
        r#"CREATE INDEX if not exists {}_idx ON {} (
         ID,
         State
     );"#, name, name)
}
fn create_main() -> String {
    String::from(r#"CREATE TABLE IF NOT EXISTS SexOffender (
        id Integer,
        name,
        age,
        dateOfBirth,
        state,
        aliases,
        offenses,
        addresses,
        photos,
        personalDetails )"#)
}


fn create_photos() -> String {
    String::from(r#"CREATE TABLE IF NOT EXISTS Photos (
        id INTEGER,
        name TEXT,
        size Integer,
        data Blob,
        state TEXT
    )"#)
}

fn create_db(conn: &Connection) -> Result<(), Box<Error>> {

    set_pragmas(conn);
    conn.execute(create_main().as_str(), NO_PARAMS).expect("Unable to create main");
    conn.execute(create_photos().as_str(), NO_PARAMS).expect("unable to create photos");
    conn.execute(create_default_index("SexOffender").as_str(), NO_PARAMS).expect("unable to create main index");
    conn.execute(create_default_non_unique_index("Photos").as_str(), NO_PARAMS).expect("Unable to create photos index");
    Ok(())
}

fn set_pragmas(conn: &Connection) {
//
  conn.pragma_update(None, "journal_mode",&String::from("OFF")).expect("Unable to set Pragma");
    //conn.execute("PRAGMA journal_mode=OFF", NO_PARAMS).expect("Unable to set PRAGMA");
    let x = "";
}

fn create_table_query(reader: &mut csv::Reader<File>, tname: &str) -> Result<String, Box<Error>> {
    let mut q = format!("CREATE TABLE if not exists {} (", tname);
    //add the header names as column names for out table.
    let mut create_table = reader.headers().unwrap().iter().fold(q, |acc, head| {
        format!("{} {}, ", acc, head)
    });
    create_table.push_str("state )"); //add our extra state field and close the ()
    //println!("----------------------------------------");
    //println!("{}", create_table);
    //println!("----------------------------------------");

    Ok(create_table)
}

fn create_blob_table_query() -> Result<String, Box<Error>> {
    Ok(String::from("CREATE TABLE if not exists Photos (id,name, size, data)"))
}

fn create_insert_query(reader: &mut csv::Reader<File>, tname: &str) -> Result<String, Box<Error>> {
    //being constructing the insert statement.
    let mut q = format!("INSERT INTO {} ( ", tname);

    //add the headers as columns
    let mut insert_query = reader.headers().unwrap().iter().fold(q, |acc, head| {
        format!("{} {}, ", acc, head)
    });

    //create the raw data tables. essentialy a copy of whatever crayzee
    //csv format we get.
    insert_query.push_str("state ) VALUES ("); //add last field and open VALUES.
    //let chars_to_trim: &[char] = &[' ', ','];

    //add value parameter placeholders
    let header_count = reader.headers().unwrap().len();
    for i in 0..header_count {
        insert_query.push_str("?,");
    }

    insert_query.push_str("?)"); //our state parameter.

    //println!("------------------------------------");
    //println!("{}", insert_query);
    //println!("------------------------------------");

    Ok(insert_query)
}

use super::downloader::ExtractedFile;
use core::borrow::{Borrow, BorrowMut};

pub fn prepare_import() -> Result<(), Box<Error>> {
    let conn = Connection::open(SQL_PATH)?;

    create_db(&conn).expect("something didn't create good.");
    conn.close();
    Ok(())
}

pub fn import_data(extracted_file: &ExtractedFile) -> Result<(), Box<Error>> {
    use ExtractedFile::*;
    let conn = Connection::open(SQL_PATH)?;

    match extracted_file {
        Csv { path, state } => {

            //this should be filtered out, really.
            let dsp = path.display().to_string();
            if dsp.contains("screenshot") { //|| dsp.contains("photos") {
                return Ok(());
            }
            let file = File::open(path)?; //.unwrap();
            let mut csv_reader = open_csv_reader(file)?;

            let table_name = String::from(path.file_stem().unwrap().to_str().unwrap());
            //let table_name = table_name.split("_").collect::<Vec<&str>>()[1];
            let mut table_query = create_table_query(&mut csv_reader, &table_name)?;
            conn.execute(&table_query, NO_PARAMS)?;

            let insert_query = create_insert_query(&mut csv_reader, &table_name)?;
            conn.execute("Begin Transaction;", NO_PARAMS);

            //insert a record (line of csv) into sqlite table.
            for result in csv_reader.byte_records() {//.records() {
                match result  {  //.expect("unable to get csv record");
                    Ok(record) => {
//                        let mut csv_values: Vec<&str> = record.iter().collect();
                        //record.push_field()
                        //let mut csv_values: &[u8] = record.iter().collect();
                        let mut rec = record.clone();
                        rec.push_field(state.as_bytes());
                       // csv_values.push_field(state.as_bytes());
                        let res = conn.execute(&insert_query, &rec);
                        //println!("{:?}", rec);

                    },
                    Err(e) => {
                        println!("Row data error: {}", e);
                    }
                }
                /*if let Err(e) = res {
                    println!("conn error: {:?}", e);
                }*/
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
                //println!("name: {} size: {}", img_name.display(), img_size);
                let name = img_name.display().to_string();
                let idx = if let Some(p) = name.find('_') {
                    p
                } else {
                    name.find('.').expect("a damn index")
                };

                let photo_id = &name[0..idx];
                std::io::copy(&mut img_file, &mut blob);

                let r = conn.execute("INSERT into photos (id,name, size, data,state) VALUES (?,?,?,?,?)",
                                     params![photo_id,name,img_size, blob,state]).expect("A damn image import");

                blob.clear();
            }

            conn.execute("COMMIT TRANSACTION;", NO_PARAMS);
        }
    };

    conn.close();
    Ok(())
}

/*
fn import_csv_file(conn: &Connection, reader: &mut csv::Reader) -> Result<(), Box<Error>> {

    let table_name = String::from(pth.file_stem().unwrap().to_str().unwrap());
    let mut table_query = create_table_query(&mut reader, &table_name)?;
    conn.execute(&table_query, NO_PARAMS)?;

    let insert_query = create_insert_query(&mut reader, &table_name)?;
    conn.execute("Begin Transaction;", NO_PARAMS);

    //insert a record (line of csv) into sqlite table.
    for result in reader.records() {
        let record = result?;
        let csv_values: Vec<&str> = record.iter().collect();
        conn.execute(&insert_query, &csv_values)?;
    }
    conn.execute("COMMIT TRANSACTION;", NO_PARAMS);



    Ok(())
}
*/
//reads a csv file and imports the data into a sqlite db.
//the filename is the name of the table. easy.
pub fn import_csv_file2(path: &Path) -> Result<(), Box<Error>> {
    let lpath = path::Path::new(path);
    let conn = Connection::open(SQL_PATH)?;
    let file = File::open(lpath).unwrap();
    let mut csv_reader = open_csv_reader(file)?;
    let table_name = String::from(lpath.file_stem().unwrap().to_str().unwrap());
    let mut table_query = create_table_query(&mut csv_reader, &table_name)?;
    conn.execute(&table_query, NO_PARAMS)?;

    let insert_query = create_insert_query(&mut csv_reader, &table_name)?;
    conn.execute("Begin Transaction;", NO_PARAMS);

    //insert a record (line of csv) into sqlite table.
    for result in csv_reader.records() {
        let record = result?;
        let csv_values: Vec<&str> = record.iter().collect();
        conn.execute(&insert_query, &csv_values)?;
    }
    conn.execute("COMMIT TRANSACTION;", NO_PARAMS);

    Ok(())
}

//insert all the image files into a database table. Sounds wrong but it
//oh so right.
pub fn import_images(path: &Path) -> Result<(), Box<Error>> {
    let lpath = path::Path::new(path);
    //lpath.iter().for_each()
    Ok(())
}




