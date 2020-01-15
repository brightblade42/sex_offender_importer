use regex::{self,  Regex};

use crate::config::{PathVars, Env};
use rusqlite::Connection;
use std::fs::File;

pub type GenError = Box<dyn std::error::Error>;
pub type GenResult<T> = ::std::result::Result<T, GenError>; //not bip bip bip.. Lelu.

pub static IMPORT_LOG: &'static str = "/opt/eyemetric/sex_offender/app/importlog.sqlite";
pub static SQL_FOLDER: &'static str = "/opt/eyemetric/sex_offender/app/sql";

///Removes junk characters and leave only the cleanest and choicest ascii characters.
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
///TODO: figure out if caching the connection or using some connection pool crate would
///be better. Probably
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

///Some fields are called `state` and that interferes with our custom state field.
///this function returns a new field called Addr_State
pub fn convert_state_field(field: &str) -> &str {

    if field == "State" || field == "state" {
        "Addr_State"
    } else {
        field
    }
}

///examines the field string to determine if it violates
///sql column naming rules and replaces bad field with good field.
///Currently the only invalid names that have been discovered
///are names containing `from` and `to`.
pub fn convert_invalid_field_name(field: &str) -> &str {

    match field {
        "from" => "from_",
        "to" => "to_",
        _ => field
    }
}

///examines a field for whitespace which is not allowed and
/// replaces them with _ underscores.
pub fn convert_space_in_field(field: &str) -> String {

    if field.trim().contains(" ") {
        field.replace(" ", "_")
    } else {
        field.to_owned()
    }
}

///some date fields contain more than just the date string,
///sometimes it's junk and sometimes it's other formatted data.
///we need dates, nothing else.
pub fn format_date(date_str: &str) -> &str {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d{2}/\d{2}/\d{4}").unwrap();
    }

    if RE.is_match(date_str) {
        let c = RE.captures(date_str).unwrap();
        c.get(0).unwrap().as_str()
    } else {
        date_str
    }
}

