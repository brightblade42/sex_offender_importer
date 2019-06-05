//extern crate csv;
use std::path;
use std::io::{BufReader, BufWriter, Read, Write};
use std::error::Error;
use csv;
use std::fs;
use std::fs::{File};

use rusqlite::{Connection, params, NO_PARAMS, ToSql};
use std::path::Path;
use std::string::ToString;

use zip;
use zip::ZipArchive;

static SQL_PATH: &'static str = "/home/d-rezzer/dev/ftp/sexoffenders.sqlite";



fn open_csv_reader(file: File) -> Result<csv::Reader<File>, Box<Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .from_reader(file);

    Ok(rdr)
}

fn create_default_index(name: &str) -> String {
    format!(
        r#"CREATE INDEX {}_idx ON {} (
         ID,
         State
     );"#, name,name)

}
fn create_main() -> String {

    //n.split("_").collect::<Vec<&str>>();
    String::from(r#"CREATE TABLE IF NOT EXISTS main (
         ID,
         DOB,
         DateOfBirth,
         DriversLicenseStateNumber,
         Address,
         Eyes,
         Hair,
         Name,
         Race,
         ScarsTattoos,
         Scars_Tattoos,
         Sex,
         Age,
         Height,
         Level,
         RiskLevel,
         Status,
         Weight,
         State
     )"#)

}

fn create_addresses() -> String {
    String::from(r#"CREATE TABLE IF NOT EXISTS  addresses (
         id,
         type,
         address1,
         address2,
         name,
         state
     )"#)

}

fn create_aliases() -> String {
    String::from(r#"CREATE TABLE IF NOT EXISTS aliases (
        id,
         alias,
         dob,
         age,
         state
     )"#)

}

fn create_offenses() -> String {
    String::from(r#"CREATE TABLE IF NOT EXISTS offenses (
         id,
         offense,
         description,
         date_convicted,
         conviction_state,
         release_date,
         details,
         state
     )"#)

}

fn create_db(conn: &Connection) -> Result<(), Box<Error>> {

    //let conn = Connection::open(SQL_PATH)?;
   // conn.execute("BEGIN TRANSACTION", NO_PARAMS);
    conn.execute(create_main().as_str(), NO_PARAMS).expect("Unable to create main");
    conn.execute(create_addresses().as_str(), NO_PARAMS).expect("unable to create addresses");
    conn.execute(create_aliases().as_str(), NO_PARAMS).expect("unable to create aliases");
    conn.execute(create_offenses().as_str(), NO_PARAMS).expect("unable to create offenses");
    conn.execute(create_default_index("main").as_str(), NO_PARAMS).expect("unable to create main index");
    conn.execute(create_default_index("addresses").as_str(), NO_PARAMS);
    conn.execute(create_default_index("aliases").as_str(), NO_PARAMS);
    conn.execute(create_default_index("offenses").as_str(), NO_PARAMS);
    //conn.execute("COMMIT TRANSACTION", NO_PARAMS);
    Ok(())
}

fn create_table_query(reader: &mut csv::Reader<File>, tname: &str) -> Result<String, Box<Error>> {

    let mut q = format!("CREATE TABLE if not exists {} (", tname);

    //add the header names as column names for out table.
    let mut create_table = reader.headers().unwrap().iter().fold(q, |acc, head| {
        format!("{} {}, ", acc, head)
    });

    create_table.push_str("state )"); //add our extra state field and close the ()
    //get rid of trailing comma
    //let chars_to_trim: &[char] = &[' ', ','];
   // let trimmed_str: &str = create_table.trim_matches(chars_to_trim);
   // let mut create_table = format!("{})", trimmed_str);
     //let mut create_table = format!("{})", create_table);
    println!("----------------------------------------");
    println!("{}", create_table);
    println!("----------------------------------------");


    Ok(create_table)
}

fn create_blob_table_query() -> Result<String, Box<Error>> {
   Ok(String::from("CREATE TABLE if not exists Photo (name, size, data)"))

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
    let chars_to_trim: &[char] = &[' ', ','];
    //remove trailing comma
    //let trimmed_iq: &str = insert_query.trim_matches(chars_to_trim);

    //begin the Value part of insert query
    //let mut insert_query = format!("{}) VALUES (", trimmed_iq);

    //add value parameter placeholders
    let header_count = reader.headers().unwrap().len();
    for i in 0..header_count {
        insert_query.push_str("?,");
    }

    insert_query.push_str("?)"); //our state parameter.
    //remove training comma
//    let mut insert_query = String::from(insert_query.trim_matches(chars_to_trim));
//    insert_query.push_str(")");
    println!("------------------------------------");
    println!("{}", insert_query);
    println!("------------------------------------");

    Ok(insert_query)
}

use super::downloader::{ExtractedFile};
use core::borrow::Borrow;
pub fn prepare_import() -> Result<(), Box<Error>> {
  let conn = Connection::open(SQL_PATH)?;

    create_db(&conn).expect("something didn't create good.");
    conn.close();
    Ok(())
}
pub fn import_data(extractedfile: &ExtractedFile) -> Result<(), Box<Error>> {

    use ExtractedFile::*;
    let conn = Connection::open(SQL_PATH)?;

    match extractedfile {

        Csv { path, state}  => {

            //this should be filtered out, really.
            let dsp = path.display().to_string();
            if dsp.contains("screenshot") || dsp.contains("photos") {
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
            for result in csv_reader.records() {
                let record = result.expect("unable to get csv record");
                let mut csv_values: Vec<&str> = record.iter().collect();
                csv_values.push(state.as_str());
                let res = conn.execute(&insert_query, &csv_values);

                if let Err(e) = res {
                    println!("conn error: {:?}", e);
                }
            }
            conn.execute("COMMIT TRANSACTION;", NO_PARAMS);
        },

        ImageArchive {path, state } => {
            ()
/*

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

            for i in 0 .. archive.len() {

                let mut img_file = archive.by_index(i)?;

                let img_name = img_file.sanitized_name();
                let img_size = img_file.size() as u32;

                println!("name: {} size: {}", img_name.display(), img_size);

                let name = img_name.display().to_string();
                //let mut bufW = BufWriter::new(blobby_mcgee);
                std::io::copy(&mut img_file, &mut blob);

               let r = conn.execute("INSERT into PHOTO (name, size, data) VALUES (?,?,?)",
                            params![name,img_size, blob ])?;

                blob.clear();


            }

            conn.execute("COMMIT TRANSACTION;", NO_PARAMS);
*/
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




