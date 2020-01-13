use std::{
    path::{PathBuf},
    io::{BufWriter, BufReader},
    fs::{self, File},
    ffi::OsStr,
};

use crate::util::{
    GenResult
};


use crate::config::PathVars;
use crate::importer::{ExtractedFile, csv::Csv, img::ImageArchive};


pub enum ExtractOptions {
    Default,
    ImagesOnly,
    CSVOnly
}

pub struct Extractor<'a> {
    config: &'a PathVars,
}

impl Extractor<'_> {

    pub fn new(config: &PathVars) -> Extractor {
        Extractor {
            config,
        }
    }

    ///Takes a path to an archive file, extracts its contents and return:ws
    /// a List of ExtractedFile objects that describe the contents.
    /// An Extracted file can be one of two variants. Csv or ImageArchive
    pub fn extract_archive(&mut self, archive_path: PathBuf, options: &ExtractOptions, overwrite: bool) -> GenResult<Vec<ExtractedFile>> {
        let mut extracted_files: Vec<ExtractedFile> = Vec::new(); //store our list of csv files.

          match options {
                  ExtractOptions::CSVOnly => {
                      if archive_path.to_string_lossy().contains("images") {
                          println!("Importing just csv");
                          return Ok(vec![]);
                      }
                  },
                  ExtractOptions::ImagesOnly => {

                      if archive_path.to_string_lossy().contains("records") {
                          println!("Importing just images");
                          return Ok(vec![]);
                      }
                  },
                  _ => {
                      println!("importing all the things");
                  }
              }


        let state_abbrev = &archive_path.file_name().unwrap().to_str().unwrap()[..2];
        let archive_file = BufReader::new(File::open(&archive_path)?);
        let mut archive = zip::ZipArchive::new(archive_file)?;

        for i in 0..archive.len() {

            let mut embedded_file = archive.by_index(i)?;
            let embedded_file_name = embedded_file.sanitized_name();
            if self.is_file_blacklisted(&embedded_file_name.to_str().unwrap()) {
                continue; //move along to zee next file
            }

            let extracts_path = self.get_extract_path(&state_abbrev, &archive_path, embedded_file_name.as_os_str());
            if overwrite || !extracts_path.exists() {
                let mut outfile = BufWriter::new(File::create(&extracts_path)?);
                std::io::copy(&mut embedded_file, &mut outfile)?;
            }
            match embedded_file_name.extension() {

                Some(ext) if ext == "csv" => {
                    let ex_file = ExtractedFile::Csv(Csv { path: extracts_path, state: String::from(state_abbrev), delimiter: '|' });
                    extracted_files.push(ex_file);
                }

                Some(ext) if ext == "txt" => {
                    let delim = if state_abbrev == "TX" { '\t' } else { ','};
                    let ex_file = ExtractedFile::Csv(Csv{ path: extracts_path, state: String::from(state_abbrev), delimiter: delim });
                    println!("txt file: {:?}", &ex_file);
                    extracted_files.push(ex_file);
                }

                //images are stored as embedded zip files of the main archive file.
                Some(ext) if ext == "zip" => {
                    let ex_file = ExtractedFile::ImageArchive(ImageArchive { path: extracts_path, state: String::from(state_abbrev) });
                    extracted_files.push(ex_file);
                }
                None => {
                    println!("No file extension found in extracted file!");
                }
                _ => {
                    println!("Unsupported file extension found and ignored.");
                }
            }
        }
        Ok(extracted_files)
    }
    ///After a file is extracted, it is saved to a configured Extraction path.
    ///This function builds up the path where Extracted files will be saved before their contents
    /// are parsed and imported.
    fn get_extract_path(&self, state: &str, archive_path: &PathBuf, file_name: &OsStr) -> PathBuf {

        let mut extracts_path = self.config.extracts_path();
        extracts_path.push(state);

        if archive_path.to_string_lossy().contains("records") {
            extracts_path.push("records");
        } else {
            extracts_path.push("images");
        }

        fs::create_dir_all(&extracts_path).expect("Unable to create extraction path");
        extracts_path.push(file_name);

        extracts_path
    }

    ///There are a lot of files that we don't care about.
    ///This function takes a file name and checks it against the current blacklist.
    ///If there is a match then return true else false.
    fn is_file_blacklisted(&self, name: &str) -> bool {

        let black_list = vec![
            "addressevent",
            "agency","col",
            "counties",
            "education",
            "educationevent",
            //"indv",  turns out indv is used to match texas photo ids to photos.
            "indv_sor",
            "institute",
            "institutecampus",
            "occupationallicense",
            "occupationallicenseevent",
            "occupationallicenseissuers",
            "occupationallicenselicenses",
            "placecodes",
            "photo2",
            "registrationevent",
            "sqlbcp",
            "tablecodes",
            "tbl",
            "val"
        ];

        let black_listed = black_list.iter().any(|&x| name.to_lowercase().as_str().contains(x));

        black_listed

    }
}
