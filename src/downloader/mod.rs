pub mod records;
pub mod archives;

use records::{RecordStatus, RecordInfo, FileInfo};

use ftp::{
    FtpStream,
    types::FileType,

};
use std::{
    str,
    fs::{self, File},
    io::{BufWriter, Write},
    iter::Iterator,
    path::PathBuf,
    convert::From,
    clone::Clone,
};

use rusqlite::{self, Connection, params,  Statement};
use serde_rusqlite::to_params_named;
use crate::config::Config;

use crate::util::{
    self,
    GenResult,
    GenError,
    IMPORT_LOG,
};

use archives::SexOffenderArchive;
use crate::downloader::records::ImageInfo;
//use std::fs::DirBuilder;

static SEX_OFFENDER_PATH: &str = "";
const CHUNK_SIZE: usize = 2048;

///Downloader connects to an ftp server,
// downloads
pub struct Downloader {
    stream: FtpStream,
    config: Config,
    conn: Connection,
}

pub enum DownloadOption {
    OnlyNew,
    Always,
}

impl Downloader {

    ///create the Downloader object, connect and login to ftp server
    ///After that we're ready to query the server for available files and begin downloading.
    pub fn connect(config: &Config) -> GenResult<Self> {
        let mut dloader = Self {
            stream: FtpStream::connect(config.address)?,
            config: config.clone(), //why do we need ths?
            conn: Connection::open(util::IMPORT_LOG).expect("A data connection"),
        };

        dloader.stream.login(&config.name, &config.pass).expect("Unable to login to ftp site");

        Ok(dloader)
    }

    ///end ftp session
    pub fn disconnect(&mut self) {
        self.stream.quit().expect("Unable to quite ftp Stream");
    }

    ///download the archive and transform to a SexOffenderArchive object
    pub fn download_file(&mut self, file_info: &FileInfo) -> GenResult<SexOffenderArchive> {

        println!("downloading {}  ... ", file_info.name());

        self.start_download_log(file_info)?;
        let res = self.save_archive(file_info)?; //.expect(&format!("Unable to complete saving archive {}", file_info.name()));

        println!("{:?}", file_info);

        Ok(res)

    }

    ///returns a list of ALL available files for download from remote server.
    /// These may or may not be newer than what we already have
    ///a filter can be passed in to narrow the list.
    ///This will update our available log if there have been updates since we last checked.
    pub fn get_all_available_file_list(&mut self, filter: fn(&String) -> bool, _file_opt: DownloadOption) -> GenResult<Vec<GenResult<FileInfo>>> {
        let remote_base_path = self.config.ftp_base_path; 
        let sex_offender_path = self.config.ftp_sex_offender_path; 

        let state_folders = self.stream.nlst(Some(remote_base_path)).expect("Unable to get a remote file listing");

        /*println!("ftp paths");
        println!("{}", &remote_base_path);
        println!("{}", &sex_offender_path);
*/
        let hard_filter = |x: &String| !x.contains(".txt") && !x.contains("united_states");

        let available_files: Vec<GenResult<FileInfo>> = state_folders
            .into_iter()
            .map(|state_folder| {
                let sex_offender_folder = format!("{}{}", state_folder, sex_offender_path);

                let file_list: Vec<GenResult<FileInfo>> = self.stream
                    .list(Some(&sex_offender_folder)) //list remote dir
                    .into_iter()
                    .flatten()
                    .filter(hard_filter) //base filter
                    .filter(filter) //user provided
                    .map(|line| self.create_file_info(&state_folder, &line))
                    .collect();

                file_list
            })
            .flatten()
            .collect();

        self.log_available_updates(&available_files)?;

        Ok(available_files)
    }


    ///Returns the list of files that have not changed since the last
    ///update.
    pub fn get_unchanged_list(&self) -> Vec<GenResult<FileInfo>> {
       let mut qry = self.conn.prepare("select * from current_available where name in (select name from download_log where status='Success')  order by last_modified desc")
           .expect("Unable to access data store");

        let mut file_list = self.build_file_info_list(&mut qry);
        file_list

    }
    ///Filters out downloads that have already completed successfully.
    ///We may not get all the files in a single session. If we open a new session
    ///we want to make sure that we don't try to download everything again.
    pub fn get_newest_available_list(&self) -> Vec<GenResult<FileInfo>>  {

        let mut qry = self.conn.prepare("SELECT * from current_available WHERE name not in (Select name from download_log where status='Success')")
            .expect("Unable to get newest update list from db");

        self.build_file_info_list(&mut qry)
    }

    ///executes a sqlite statement and returns a list of FileInfo values.
    fn build_file_info_list(&self, stmt: &mut Statement<'_>) -> Vec<GenResult<FileInfo>>  {

        let mut file_list: Vec<GenResult<FileInfo>> = Vec::new();

        let res = stmt.query_map([], |row| {
            let rpath: String = row.get(0)?;
            let name: String =   row.get(1)?;
            let lmod = row.get(2)?;
            let size = row.get(3)?;
            let status = RecordStatus::None; //TODO: this isn't really used so think   //row.get(4)?;

            let fi = if name.contains("images") {
                FileInfo::Image(ImageInfo {
                    rpath: Some(rpath),
                    name: Some(name),
                    last_modified: Some(lmod),
                    size: Some(size),
                    status
                })
            } else {
                FileInfo::Record(RecordInfo {

                    rpath: Some(rpath),
                    name: Some(name),
                    last_modified: Some(lmod),
                    size: Some(size),
                    status
                })
            };

            Ok(fi)
        });

        for x in res.unwrap() {
            file_list.push(Ok(x.unwrap()));
        }

        file_list

    }

    ///Transform a line of ftp file info to a FileInfo struct
    fn create_file_info(&self, remote_file_path: &str, ftp_line: &str) -> GenResult<FileInfo> {
        //println!("{}", ftp_line);
        let mut line = ftp_line.split_whitespace();
        //not sure about this.
        if line.clone().count() == 0 {
            return Err(GenError::from(format!("Unable to create FileInfo for {}", remote_file_path)));
        }
        let size = line.nth(4).unwrap();
        let name = line.last().unwrap();
        let date_str = self.get_date_from_name(name);

        let fin = FileInfo::Record(RecordInfo {
            rpath: Some(remote_file_path.to_string()),
            name: Some(name.to_string()),
            last_modified: Some(date_str),
            size: Some(size.parse().unwrap()),

            status: RecordStatus::None, //None is the beginning state.
        });

        Ok(fin)
    }

    ///a date is embedded in the archive file name so we parse that out
    ///and return a formatted date string.
    fn get_date_from_name(&self, name: &str) -> String {

        let mut nsplit: Vec<&str> = name.split('_').collect();
        nsplit.reverse();
        let y = nsplit.get(4).unwrap();
        let m = nsplit.get(3).unwrap();
        let d = nsplit.get(2).unwrap();

        format!("{}-{}-{}", y,m,d)
    }

   ///return the Path representing the remote ftp path to the fileInfo argument
    fn get_remote_path(&self, file_info: &FileInfo) -> PathBuf {

        let pb = match file_info {
            FileInfo::Record(r) => {
                r.rpath.as_ref().unwrap()
            }
            FileInfo::Image(i) => {
                i.rpath.as_ref().unwrap()
            }
        };

        PathBuf::from(format!("{}{}", pb, &self.config.ftp_sex_offender_path)) 

    }

    ///downloads the remote archive file and writes to disk.
    fn save_archive(&mut self, file_info: &FileInfo) -> GenResult<SexOffenderArchive> {
        let file_name = file_info.name();
        let archive_base = &self.config.archives_path; //self.config.archive_path();
        let archive_file_path = archive_base.join(&file_info.name()); 
        //let archive_file_path = self.config.archive_file_path(&file_info.name());

        //make sure we're setup to dload binary files
        self.stream.transfer_type(FileType::Binary)?;
        let remote_path = self.get_remote_path(&file_info);

        //change ftp dir.
        self.stream.cwd(remote_path.to_str().unwrap()).expect("could not change ftp dir"); //.is_err() {
        println!("dir change success");

        let conn = Connection::open(IMPORT_LOG).expect("Unable to open Connection");

        let update_log = |byte_count: usize | {

            let bytes = byte_count as i64;
            conn.execute("UPDATE download_log set bytes_downloaded=? where name=?",
                                       params![bytes, file_info.name()]).expect("Unable to log byte count");
        };

        let update_download_status = |status: &str| {

            let rc = conn.execute("UPDATE download_log set status=? where name=?",
                                  params![status, file_info.name()]).expect("Unable to log byte count");

            println!("dbcode: {} status: {}", rc, status);
        };

        //FTP RETR command = download
        let res = self.stream.retr(&file_name, |stream| {

            fs::create_dir_all(&archive_base).expect("Unable to create archive dir");
            println!("local base dir: {}", archive_base.display());
            let mut archive_writer = BufWriter::new(File::create(&archive_file_path).expect("Unable to write archive to disk"));

            let mut total_bytes: usize = 0;
            let mut buff: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

            let mut bytes_read = stream.read(&mut buff).expect("Unable to read bytes");
            total_bytes += bytes_read;

            while bytes_read > 0 {
                archive_writer.write_all(&buff[..bytes_read]).expect("Unable to write bytes");
                bytes_read = stream.read(&mut buff).expect("Unable to read stream");
                total_bytes += bytes_read;
                update_log(total_bytes);
            }
            //TODO: if the file size from the server doesn't match
            //the total bytes we've received then we didn't get the whole file
            //das a problem. I wonder if the server supports ftp resume.
            match  archive_writer.flush() {
                Ok(()) => {
                    println!("bytes written:: {}", total_bytes);
                    update_download_status("Success");
                }
                Err(e) => {
                    println!("bad mojo {}", e);
                    update_download_status("Failure");
                    //TODO queue for a resume if possible.
                }
            }

            Ok(SexOffenderArchive {
                path: archive_file_path.clone(),
                size: total_bytes,
            })
        });

        Ok(res.unwrap()) //workaround for error handling I don't understand.

    }

    ///log the list of available archives we received from the server.
    fn log_available_updates(&self, remote_files: &[GenResult<FileInfo>]) -> GenResult<()> {

        self.conn.execute(
                "CREATE TABLE if not exists current_available (rpath, name, last_modified, size integer, status) ",
                [],
            )?;

        self.conn.execute("BEGIN TRANSACTION", [])?; //.expect("Unable to start transaction");
        //clear out
        self.conn.execute("DELETE FROM current_available", [])?; //.expect("Unable to delete from current_available")?;
        for r_file in remote_files {
            if let Ok(FileInfo::Record(ri)) = r_file {
                self.conn.execute_named(r#"INSERT INTO current_available (rpath, name, last_modified, size, status)
                                       VALUES (:rpath, :name, :last_modified, :size, :status )"#,
                                        &to_params_named(ri).unwrap().to_slice()).expect("Unable to insert row into temp table");
            }
        }

        self.conn.execute("COMMIT TRANSACTION", [])?; //.expect("Failed to Commit Transaction!");

        Ok(())
    }

    ///begins the logging process that tracks the progress of our downloads.
    ///Adds a new log entry with a status of "InFlight"
    fn start_download_log(&self, file_info: &FileInfo) -> GenResult<()>  {

        match file_info {
            FileInfo::Record(record_info) => {

                self.conn.execute("Delete from download_log where name=? and last_modified=?", params![record_info.name, record_info.last_modified])?;
                self.conn.execute("INSERT INTO  download_log (name, last_modified, size, bytes_downloaded, status) VALUES (?,?,?,?,?)",
                                           params![record_info.name, record_info.last_modified, record_info.size, 0, "InFlight" ])?;
            }
            FileInfo::Image(record_info) => {

                self.conn.execute("Delete from download_log where name=? and last_modified=?", params![record_info.name, record_info.last_modified])?;
                self.conn.execute("INSERT INTO  download_log (name, last_modified, size, bytes_downloaded, status) VALUES (?,?,?,?,?)",
                                           params![record_info.name, record_info.last_modified, record_info.size, 0, "InFlight" ])?;
            }
        }

        Ok(())
    }


    pub fn rebuild_log_from_archives(&self) -> GenResult<()>{

        let expath = fs::read_dir(&self.config.archives_path)?; //archive_path())?;


        self.conn.execute("BEGIN TRANSACTION;", []);
        self.conn.execute("Delete from download_log", []);
        for entry in expath {
            let item = &entry?;
            let meta = item.metadata()?;

            if meta.is_file() {
                let name = item.file_name().to_str().unwrap().to_string();
                let last_mod =  self.get_date_from_name(item.file_name().to_str().unwrap());
                //println!("mod: {}", last_mod);
                let size = meta.len().to_string();

                self.conn.execute("Insert into download_log (name, last_modified, size, bytes_downloaded, status) VALUES (?,?,?,?,?)",
                                    params![name, last_mod, size, size, "Success"])?;

            }

        }
        self.conn.execute("END TRANSACTION;", []);

        Ok(())

    }
}

