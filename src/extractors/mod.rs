
use std::{
    path::{PathBuf, Path},
    io::{BufWriter, BufReader},
    fs::{self, File},
    ffi::OsStr,
};
use serde::{Serialize, Deserialize};
use crate::util::GenResult ;
use crate::importer::{Import, csv_importer::Csv, img::ImageArchive};
use crate::config::Config;

///We can choose whether to extract ALL files (Default), Images only, or Csv only.
pub enum ExtractOptions {
    Default,
    ImagesOnly,
    CSVOnly
}

///Reaches into the soul of a zip file and extracts the csv files and Images
///and saves them to disk for the importer. 
pub struct Extractor<'a> {
    config: &'a Config,
}
impl Extractor<'_> {
    pub fn new(config: &Config) -> Extractor {
        Extractor {
            config,
        }
    }

    ///Takes a path to an archive file, examines the embedded files, extracts its contents, writes
    /// the embedded files to disk and returns a Vec of ExtractedFile types
    ///  ExtractedFile is a type that describes aspects of the embedded file (location, state, deliteter
    /// character. An Extracted file can be one of two variants. Csv or ImageArchive
    pub fn extract_archive(&mut self, archive_path: PathBuf, options: &ExtractOptions, overwrite: bool) 
        -> GenResult<Vec<ExtractedFile>> {

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

        //TODO: this is unsettling. 
        let state_abbrev = &archive_path.file_name().unwrap().to_str().unwrap()[..2];
        let archive_file = BufReader::new(File::open(&archive_path)?);
        let mut archive = zip::ZipArchive::new(archive_file)?;

        for i in 0..archive.len() {

            let mut embedded_file = archive.by_index(i)?;
            let embedded_file_name = embedded_file.sanitized_name();
            if self.is_file_blacklisted(embedded_file_name.to_str().unwrap()) {
                continue; //move along to zee next file
            }

            let extracts_path = self.get_extract_path(state_abbrev, &archive_path, embedded_file_name.as_os_str());
            if overwrite || !extracts_path.exists() {
                let mut outfile = BufWriter::new(File::create(&extracts_path)?);
                std::io::copy(&mut embedded_file, &mut outfile)?;
            }

            //determine the method of extraction based on the file extension
            match embedded_file_name.extension() {

                Some(ext) if ext == "csv" => {
                    let ex_file = ExtractedFile::Csv(Csv { path: extracts_path, state: String::from(state_abbrev), delimiter: '|' });
                    extracted_files.push(ex_file);
                }

                //some csv files are called txt files. 
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
    fn get_extract_path(&self, state: &str, archive_path: &Path, file_name: &OsStr) -> PathBuf {

        let extracts_path = &mut self.config.extracts_path.to_path_buf(); 
        extracts_path.push(state);

        //what kind of an archive are we working with , csv or image
        if archive_path.to_string_lossy().contains("records") {
            extracts_path.push("records");
        } else {
            extracts_path.push("images");
        }

        fs::create_dir_all(&extracts_path).expect("Unable to create extraction path");
        extracts_path.push(file_name);

        println!("===================");
        println!("Extracts path");
        println!("{:?}", extracts_path);
        println!("===================");

        extracts_path.to_path_buf()
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
///ExtractedFile represents either a CSV file or  embedded Zip files (image files are stored as zip files
/// within an outer zip file) that are contained within
/// the top-level zip archives downloaded from the `public data` remote server.
#[derive(Debug, Serialize, Deserialize)]
pub enum ExtractedFile {
    Csv(Csv),
    ImageArchive(ImageArchive)
}

///we implement Import trait on Extracted file to forward import calls
///to its variants.
impl Import for ExtractedFile {
    type Reader = String;

    fn open_reader(&self, _has_headers: bool) -> GenResult<Self::Reader> {
        unimplemented!()
    }

    ///forwards the import call to the specific import methods on CSV or ImageArchive,
    ///depending on which is matched.
    fn import(&self) -> GenResult<()> {
        match self {

            ExtractedFile::Csv(csv) => {
                csv.import()
            },
            ExtractedFile::ImageArchive(img) => {
                img.import()
            }
        }
    }

    //just to satisfy the trait impl
    fn import_file_data(&self) -> GenResult<()> {
        Ok(())
    }
}

