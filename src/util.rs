use regex::{self,  Regex};

use crate::config::{PathVars, Env};
use rusqlite::Connection;

pub type GenError = Box<dyn std::error::Error>;
//pub type Result<T> = ::std::result::Result<T, GenError>;
pub type GenResult<T> = ::std::result::Result<T, GenError>;


pub fn to_ascii_string(chars: &[u8]) -> String {

    let mut rstring = String::new();
    for byte in chars {
        /*if byte.is_ascii_control() {

            //rstring.push_str("");
        }*/
        if byte.is_ascii() && !byte.is_ascii_control() {
            let c = byte.clone() as char; //latin1_to_char(byte.clone());
            rstring.push(c);
        }
    }
    rstring
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

pub fn convert_space_in_field(field: &str) -> String {
    if field.trim().contains(" ") {
        field.replace(" ", "_")
    } else {
        field.to_owned()
    }
}

fn format_date(date_str: &str) -> &str {
    let reg =  Regex::new(r"\d{2}/\d{2}/d{4}").unwrap();
    if reg.is_match(date_str) {
        "coool"
    } else {
        date_str
    }
    //regex::bytes::
}
