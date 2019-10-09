use std::{
    fs::File,
    path::PathBuf,
    io::BufReader,
};

use crate::{
    util::{
        self,
        GenResult
    },
};

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
        let conn = util::get_connection(None).expect("Unable to open connection");
        self.delete_data(&conn, "Photos")?;

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
            let idx = if let Some(char_idx) = name.find('_') { char_idx } else { name.len() };
            if name.contains("table") { continue; } //not an image

            let photo_id = &name[0..idx];
            std::io::copy(&mut img_file, &mut blob)?;

            //TODO: make part of the SqlHandler trait impl.
            conn.execute("INSERT into Photos (id,name, size, data,state) VALUES (?,?,?,?,?)",
                    params![photo_id, name, img_size, blob, self.state], )?;

            blob.clear();
        }

        conn.execute("COMMIT TRANSACTION;", NO_PARAMS)?;
        Ok(())
    }
}

impl SqlHandler for ImageArchive {
    type Reader = BufReader<File>;
    fn create_table_query(&self, _reader: Option<&mut Self::Reader>, table_name: &str) -> GenResult<String>{
        Ok(format!( "CREATE TABLE if not exists {} (id,name, size, data, state)",table_name ))
    }

    fn create_insert_query(&self, _reader: &mut Self::Reader, _tname: &str) -> GenResult<String> {
        unimplemented!()
    }

    fn delete_data(&self, conn: &Connection, table_name: &str) -> Result<usize, rusqlite::Error> {
        conn.execute( &format!("DELETE FROM {} where state='{}'", table_name, &self.state), NO_PARAMS, )
    }

    fn execute(&self, _conn: &Connection) -> Result<usize, rusqlite::Error> {
        unimplemented!()
    }

}
