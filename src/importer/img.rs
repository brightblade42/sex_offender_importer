use std::{
    fs::File,
    path::PathBuf,
    io::BufReader,
};

use crate::{
    util::{
        self,
        GenResult
    }};

use super::{Import, SqlHandler};
use serde_derive::{Serialize, Deserialize};
use rusqlite::{Connection, NO_PARAMS, params};

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
        let blob_table = self.create_table_query(None, "Photos");
        let conn = util::get_connection(None).expect("Unable to open connection");
        conn.execute(&blob_table.unwrap(), NO_PARAMS)?;

        //1. we've got an archive of images. we don't want to write them
        //to disk, we want to store them as blobs in sqlite.

        //iterate images,
        //validate,
        //write to Vec<u8>
        //write bytes to db.
        let mut blob: Vec<u8> = vec![];

        let mut archive = zip::ZipArchive::new(file)?;

        conn.execute("BEGIN TRANSACTION;", NO_PARAMS)?;

        for i in 0..archive.len() {
            let mut img_file = archive.by_index(i)?;
            let img_name = img_file.sanitized_name();
            let img_size = img_file.size() as u32;
            let name = img_name.display().to_string();
            let idx = if let Some(p) = name.find('_') {
                p
            } else {
                //name.find('.').expect("a damn index")
                name.len()
            };

            if name.contains("table") { //not an image
                continue;
            }
            let photo_id = &name[0..idx];
            std::io::copy(&mut img_file, &mut blob)?;

            //println!("image: {} {} {} {}", photo_id, name, img_size, state);
            conn.execute(
                    "INSERT into Photos (id,name, size, data,state) VALUES (?,?,?,?,?)",
                    params![photo_id, name, img_size, blob, self.state],
                )?;
            //    .expect("A damn image import");

            blob.clear();
        }

        conn.execute("COMMIT TRANSACTION;", NO_PARAMS)?;

        Ok(())
    }
}

impl SqlHandler for ImageArchive {
    type Reader = BufReader<File>;
    fn create_table_query(&self, _reader: Option<&mut Self::Reader>, table_name: &str) -> GenResult<String>{
        Ok(format!( "CREATE TABLE if not exists {} (id,name, size, data)",table_name ))
    }

    fn create_insert_query(&self, _reader: &mut Self::Reader, _tname: &str) -> GenResult<String> {
        unimplemented!()
    }

    fn execute(&self, _conn: &Connection) -> Result<usize, rusqlite::Error> {
        unimplemented!()
    }
}
