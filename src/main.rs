mod sex_offender;
extern crate ftp;
mod sexoffender;
use sexoffender::SexOffenderDownloader;

//use crate::sexoffender::SexOffenderImportError;
//
fn main() {
    let mut downloader = SexOffenderDownloader::connect();

    let file_list = downloader.get_file_list();
    let mut cnt = 0;
    if file_list.is_empty() {
        println!("There was nothing new to dload!");
    }
    for file in file_list {
        match file {
            Ok(f) => {
                println!("{:?}", f);
                   let arch = downloader.get_archives(&f);
                    SexOffenderDownloader::extract_archive(&f);
                   println!("got an archive");
               }

            Err(e) => {
                println!("could not read record! {:?}", e);
            }
        }
    }



    downloader.disconnect();

    println!("all done");
}







