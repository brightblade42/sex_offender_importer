extern crate ftp;
mod sexoffender;
use std::str;
use std::io::{Cursor, Error};
use ftp::FtpStream;
use std::net::{SocketAddr, ToSocketAddrs};
use std::iter::Iterator;
use sexoffender::{SexOffenderImporter, SexOffenderImportError, FileInfo};
//use crate::sexoffender::SexOffenderImportError;

fn main() {


    let mut importer = SexOffenderImporter::connect();

    for file in importer.get_available_state_list() {
        match file {
            Ok(f) => {
                println!("{:?}", f);
            },
            Err(e) => {
                println!("could not read record!");
            }

        }
    }

    importer.disconnect();

    println!("all done");
}








