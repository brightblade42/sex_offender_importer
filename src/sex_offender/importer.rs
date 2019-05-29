//extern crate csv;
use std::path;
use std::io::{BufReader, BufWriter, Read, Write};
use std::error::Error;
use csv;
use std::fs;
use std::fs::{File};

use rusqlite::{Connection, params, NO_PARAMS};
use std::path::Path;
use std::string::ToString;

static SQL_PATH: &'static str = "/home/d-rezzer/dev/ftp/sexoffenders.sqlite";

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
}

pub fn open_csv_reader(file: File) -> Result<csv::Reader<File>, Box<Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .from_reader(file);

    Ok(rdr)
}

pub fn create_table_query(reader: &mut csv::Reader<File>, tname: &str) -> Result<String, Box<Error>> {

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

pub fn import_csv_file(path: &Path) -> Result<(), Box<Error>> {

    let lpath = path::Path::new(path);
    let conn = Connection::open(SQL_PATH)?;
    let file = File::open(lpath).unwrap();
    let mut csv_reader = open_csv_reader(file)?;
    let table_name = String::from(lpath.file_stem().unwrap().to_str().unwrap());
    let mut table_query = create_table_query(&mut csv_reader, &table_name)?;
    conn.execute(&table_query, NO_PARAMS)?;

    let insert_query = create_insert_query(&mut csv_reader, &table_name)?;
    conn.execute("Begin Transaction;", NO_PARAMS);

    for result in csv_reader.records() {
        let record = result?;
        let csv_values: Vec<&str> = record.iter().collect();
        conn.execute(&insert_query, &csv_values)?;
    }
    conn.execute("COMMIT TRANSACTION;", NO_PARAMS);

    Ok(())
}
pub fn import_images(path: &Path) -> Result<(), Box<Error>> {

    let lpath = path::Path::new(path);
    //lpath.iter().for_each()
    Ok(())
}


