use std::{
    fs::File,
    path::PathBuf,
    io::BufReader,
};

use crate::{ util::{ self, GenResult }, config::Config};
use super::{Import, SqlHandler};
use serde_derive::{Serialize, Deserialize};
use rusqlite::{Connection,  params};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageArchive {
    pub path: PathBuf,
    pub state: String,
}

impl Import for ImageArchive {
    type Reader = BufReader<File>;

    fn open_reader(&self, _has_headers: bool) -> GenResult<Self::Reader> {
        Ok(BufReader::new(File::open(&self.path).unwrap()))
    }

    fn import(&self) -> GenResult<()> {
        self.import_file_data()?;
        Ok(())
    }

    fn import_file_data(&self) -> GenResult<()> {
        let file = BufReader::new(File::open(&self.path)?);
        let config = Config::new(std::env::current_dir().unwrap());
        let conn = util::get_connection(config.offender_db).expect("Unable to open connection");

        //1. we've got an archive of images. we don't want to write them
        //to disk, we want to store them as blobs in sqlite.
        //iterate images,
        //validate,
        //write to Vec<u8>
        //write bytes to db.
        let mut blob: Vec<u8> = vec![];
        let mut archive = zip::ZipArchive::new(file)?;

        //NOTE: previous state images are deleted at beginning of import prcess
        println!("Importing images from {}", &self.path.display());

        conn.execute("BEGIN TRANSACTION;", [])?;

        let mut ins_stmt = conn.prepare("INSERT into Photos (id,name, size, data,state) VALUES (?,?,?,?,?)");

        for i in 0..archive.len() {

            let mut img_file = archive.by_index(i)?;
            let img_name = img_file.sanitized_name();
            let img_size = img_file.size() as u32;
            let name = img_name.display().to_string();
            let idx = if let Some(char_idx) = name.find('_') { char_idx } else { name.len() };
            if name.contains("table") { continue; } //not an image

            let photo_id = &name[0..idx]; //parse id from img name
            std::io::copy(&mut img_file, &mut blob)?;

            //TODO: make part of the SqlHandler trait impl.
            let _ = &ins_stmt.as_mut().unwrap()
                .execute(params![photo_id, name, img_size, blob, self.state.as_str().to_uppercase()], )?;

            blob.clear(); //we reuse the blob for the next image
        }

        conn.execute("COMMIT TRANSACTION;", [])?;
        Ok(())
    }
}

impl SqlHandler for ImageArchive {

    type Reader = BufReader<File>;
    fn create_table_query(&self, _reader: &mut Self::Reader, table_name: &str) -> GenResult<String>{
        Ok(format!( "CREATE TABLE if not exists {} (id,name, size, data, state)",table_name ))
    }

    fn create_insert_query(&self, _reader: &mut Self::Reader, _tname: &str) -> GenResult<String> {
        unimplemented!()
    }

    fn delete_data(&self, conn: &Connection, table_name: &str) -> Result<usize, rusqlite::Error> {
        conn.execute( &format!("DELETE FROM {} where state='{}'", table_name, &self.state), [], )
    }

    fn execute(&self, _conn: &Connection) -> Result<usize, rusqlite::Error> {
        unimplemented!()
    }

}
