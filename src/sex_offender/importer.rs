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

/*
fn _do_main() {
    let path = "/home/d-rezzer/dev/ftp";


    if let Ok(entries) = fs::read_dir(&path) {
        println!("hello computer:");

        for entry in entries {
            if let Ok(entry) = entry {
                let name = String::from(entry.file_name().to_str().unwrap());
                if name.ends_with("csv") {
                    let x = import_csv_file(path::Path::new(&format!("{}/{}", &path, &name)));
                    println!("err: {:?}", x);
                }
            } else {
                println!("booo");
            }
        }
    }

}*/

fn open_csv_reader(file: File) -> Result<csv::Reader<File>, Box<Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .from_reader(file);

    Ok(rdr)
}

fn create_table_query(reader: &mut csv::Reader<File>, tname: &str) -> Result<String, Box<Error>> {

    let mut q = format!("CREATE TABLE if not exists {} (", tname);

    //add the header names as column names for out table.
    let mut create_table = reader.headers().unwrap().iter().fold(q, |acc, head| {
        format!("{} {}, ", acc, head)
    });

    //get rid of trailing comma
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = create_table.trim_matches(chars_to_trim);
    let mut create_table = format!("{})", trimmed_str);
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

    let chars_to_trim: &[char] = &[' ', ','];
    //remove trailing comma
    let trimmed_iq: &str = insert_query.trim_matches(chars_to_trim);

    //begin the Value part of insert query
    let mut insert_query = format!("{}) VALUES (", trimmed_iq);

    //add value parameter placeholders
    let header_count = reader.headers().unwrap().len();
    for i in 0..header_count {
        insert_query.push_str("?,");
    }

    //remove training comma
    let mut insert_query = String::from(insert_query.trim_matches(chars_to_trim));
    insert_query.push_str(")");
    println!("------------------------------------");
    println!("{}", insert_query);
    println!("------------------------------------");

    Ok(insert_query)
}

use super::downloader::{ExtractedFile};
pub fn import_data(file: &ExtractedFile) -> Result<(), Box<Error>> {

    use ExtractedFile::*;
    let conn = Connection::open(SQL_PATH)?;

    match file {

        Csv { path, state}  => {

            let file = File::open(path)?; //.unwrap();
            let mut csv_reader = open_csv_reader(file)?;

            let table_name = String::from(path.file_stem().unwrap().to_str().unwrap());
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
        },

        ImageArchive {path, state } => {


            let blob_table = create_blob_table_query();
            conn.execute(&blob_table.unwrap(), NO_PARAMS);

            //1. we've got an archive of images. we don't want to write them
            //to disk, we want to store them as blobs in sqlite.

            //iterate images,
            //validate,
            //write to Vec<u8>
            //write bytes to db.
            let mut blobby: Vec<u8> = vec![];

            let file = BufReader::new(File::open(path).unwrap());
            let mut archive = zip::ZipArchive::new(file)?;

            conn.execute("BEGIN TRANSACTION;", NO_PARAMS);

            for i in 0 .. archive.len() {

                let mut img_file = archive.by_index(i)?;

                let img_name = img_file.sanitized_name();
                let img_size = img_file.size() as u32;

                println!("name: {} size: {}", img_name.display(), img_size);

                let name = img_name.display().to_string();
                //let mut bufW = BufWriter::new(blobby);
                std::io::copy(&mut img_file, &mut blobby);

               conn.execute("INSERT into PHOTO (name, size, data) VALUES (?,?,?)",
                            params![name,img_size, blobby ])?;

                blobby.clear();


            }

            conn.execute("COMMIT TRANSACTION;", NO_PARAMS);

        }

    };
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




