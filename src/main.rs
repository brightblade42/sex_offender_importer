extern crate ftp;
use std::str;
use std::io::Cursor;
use ftp::FtpStream;
use std::net::{SocketAddr, ToSocketAddrs};
use std::iter::Iterator;

fn main() {

    let sex_offender_path  = "/state/sex_offender";

    let mut ftp_stream = FtpStream::connect("ftptds.shadowsoft.com:21")
        .unwrap_or_else(|err| {
            panic!("{}", err);
        });

    let ll = ftp_stream.login("swg_sample", "456_sample");

    println!("Current directory {}", ftp_stream.pwd().unwrap());



    struct SexOffenderFileInfo {
        image: String,
        record: String,

    }

    let lst = ftp_stream.nlst(Some("us")).unwrap();

    #[derive(Debug)]
    struct FileInfoS(pub String, pub String, pub String, pub String);


    let available_states: Vec<FileInfo> = lst.into_iter().map(| p | {

        let mut pp = p.to_string();
        pp.push_str(sex_offender_path);

        //list the files available for this state
        //map_err?
       let res: Vec<FileInfo> = ftp_stream.list(Some(&pp))
           .into_iter()
           .flatten()
           .filter(|fi| fi.contains("records") || fi.contains("images"))
           .map(|fi| {

               create_file_info(&p, &fi)

           })
           .collect();

       res

    }).flatten().collect();


    for file in available_states {
        println!("{:?}", file);
    }

    println!("somthings happening here, what it is ain't exactly clear");

    let _   = ftp_stream.quit();

}

#[derive(Debug)]
    struct FileInfo {

        path: Option<String>,

        name: Option<String>,
        year: Option<String>,
        month: Option<String>,
        day: Option<String>,
        size: Option<String>, //convert this to i64

    }

fn create_file_info(path: &str, line: &str) -> FileInfo {

        let mut iter = line.split_whitespace().rev().take(5);

        FileInfo {
            path: Some(path.to_string()),
            name : Some(iter.next().unwrap().to_string()),
            year : Some(iter.next().unwrap().to_string()),
            month : Some(iter.next().unwrap().to_string()),
            day : Some(iter.next().unwrap().to_string()),
            size : Some(iter.next().unwrap().to_string()),
        }


}


