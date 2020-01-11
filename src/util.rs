use regex::{self,  Regex};

use crate::config::{PathVars, Env};
use rusqlite::Connection;
use std::fs::File;

pub type GenError = Box<dyn std::error::Error>;
//pub type Result<T> = ::std::result::Result<T, GenError>;
pub type GenResult<T> = ::std::result::Result<T, GenError>;

pub static IMPORT_LOG: &'static str = "/opt/eyemetric/sex_offender/app/importlog.sqlite";
pub static SQL_FOLDER: &'static str = "/opt/eyemetric/sex_offender/app/sql";

///Remove junk characters and leave only the cleanest of choice ascii characters.
pub fn to_ascii_string(chars: &[u8]) -> String {

    let mut ascii = String::new();
    for byte in chars {
        if byte.is_ascii() && !byte.is_ascii_control() {
            let c = byte.clone() as char;
            ascii.push(c);
        }
    }
    ascii
}
///returns an open sqlite connection.
///if vars Option is None then connection will be from Production config values
pub fn get_connection(vars: Option<PathVars>) -> GenResult<Connection> {

    let sql_path  = if let Some(v) = vars {
        v.sql_path()
    } else {
        let v = PathVars::new(Env::Production);
        v.sql_path()
    };

    match  Connection::open(sql_path) {
        Ok(conn) => Ok(conn),
        Err(err) => Err(GenError::from(err))
    }
}

pub fn convert_state_field(field: &str) -> &str {

    if field == "State" || field == "state" {
        "Addr_State"
    } else {
        field
    }
}

pub fn convert_invalid_field_name(field: &str) -> &str {

    match field {
        "from" => "from_",
        "to" => "to_",
        _ => field
    }
}


pub fn convert_space_in_field(field: &str) -> String {

    if field.trim().contains(" ") {
        field.replace(" ", "_")
    } else {
        field.to_owned()
    }
}

//pub fn fix_invalid_column_names()

pub fn format_date(date_str: &str) -> &str {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d{2}/\d{2}/\d{4}").unwrap();
    }
//    let reg =  Regex::new(r"\d{2}/\d{2}/\d{4}").unwrap();

    if RE.is_match(date_str) {
        let c = RE.captures(date_str).unwrap();
        c.get(0).unwrap().as_str()
    } else {
        date_str
    }
}

