extern crate ftp;
mod sexoffender;
use sexoffender::SexOffenderImporter;

//use crate::sexoffender::SexOffenderImportError;
//
fn main() {
    let mut importer = SexOffenderImporter::connect();

    let file_list = importer.get_file_list();
    let mut cnt = 0;
    for file in file_list {
        match file {
            Ok(f) => {
                println!("{:?}", f);
                   let arch = importer.get_archives(&f);
                   println!("got an archive");
               }

            Err(_e) => {
                println!("could not read record!");
            }
        }
    }



    importer.disconnect();

    println!("all done");
}







