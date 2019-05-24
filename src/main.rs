extern crate ftp;
use std::str;
use std::io::Cursor;
use ftp::FtpStream;
use std::net::{SocketAddr, ToSocketAddrs};
use std::iter::Iterator;
fn main() {
    let mut ftp_stream = FtpStream::connect("ftptds.shadowsoft.com:21")
        .unwrap_or_else(|err| {
            panic!("{}", err);
        });

    let ll = ftp_stream.login("swg_sample", "456_sample");

    println!("Current directory {}", ftp_stream.pwd().unwrap());

    //retrieve a file from server.
    //let ll = ftp_stream.cwd("test_data").unwrap();

    struct FileInfo {
        name: String,
        last_mod: String,
    }

    struct SexOffenderFileInfo {
        image: String,
        record: String,

    }

    //let lst = ftp_stream.list(Some("us")).unwrap(); //list of state paths.
    let lst = ftp_stream.nlst(Some("us")).unwrap();
    let tt: Vec<(String, String)>  = vec![("this".to_string(),"that".to_string())];

    let available_states: Vec<String> = lst.into_iter().map(| p | {
        let mut pp = p.to_string();
        pp.push_str("/state/sex_offender");


       let res: Vec<String> = ftp_stream.list(Some(&pp)).into_iter()
           .map(|item| {
               item
        }).flatten()
          .inspect(|item| {

            println!("current item {}:: {:?}", &p, item);
        })
           .collect();

        "Ssssuper!".to_string()

    }).collect();

    for file in available_states {
        println!("{:?}", file);
    }
   // println!("{:?}", lst);

    println!("somthings happening here, what it is ain't exactly clear");

    let _   = ftp_stream.quit();

}


