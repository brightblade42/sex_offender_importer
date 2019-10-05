use std::{
    io,
    path::{PathBuf}
};

use regex::{self, bytes, Regex};

use crate::config::{PathVars, Env};
use rusqlite::Connection;

pub fn to_ascii_string(chars: &[u8]) -> String {

    let mut rstring = String::new();
    for byte in chars {
        if byte.is_ascii() {
            let c = byte.clone() as char; //latin1_to_char(byte.clone());
            rstring.push(c);
        }
    }
    rstring
}

pub fn get_sql_path(vars: &PathVars) -> PathBuf {

    let sql_path = PathBuf::from(&vars.vars["app_base_path"]).join(&vars.vars["sex_offender_db"]);
    println!("sql path: {}", sql_path.to_str().unwrap());
    sql_path
}

pub fn get_root_path(vars: &PathVars) -> PathBuf {
    PathBuf::from(&vars.vars["app_base_path"]).join(&vars.vars["archives_path"])
}

pub fn get_connection() -> Result<Connection, rusqlite::Error> {
    let path_vars = PathVars::new(Env::Production);
    let sql_path = get_sql_path(&path_vars);
    Connection::open(sql_path)
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

fn format_date(dateStr: &str) -> &str {
    let reg =  Regex::new(r"\d{2}/\d{2}/d{4}").unwrap();
    if reg.is_match(dateStr) {
        "coool"
    } else {
        dateStr
    }
    //regex::bytes::
}
