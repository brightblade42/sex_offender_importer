use crate::types::{SexOffenderArchive, ExtractedFile};
use crate::config::{PathVars};
use std::path::{Path, PathBuf};
use std::io::{BufWriter, BufReader};
use std::fs::{self, File};
use std::ffi::OsStr;
use ftp::status::PATH_CREATED;


type GenError = Box<dyn std::error::Error>;
pub type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Extractor<'a> {
   config: &'a PathVars,
}

impl Extractor<'_> {
   pub fn new(config: &PathVars) -> Extractor {

      Extractor {
           config,
      }
   }
   fn generate_extract_path(&self, state: &str, archive_path: &PathBuf, file_name: &OsStr) -> PathBuf {
        let mut extracts_path = PathBuf::from(&self.config.vars["app_base_path"]).join(&self.config.vars["extracts_path"]);
        extracts_path.push(state);

        if archive_path.to_string_lossy().contains("records") {
            extracts_path.push("records");
        } else {
            extracts_path.push("images");
        }

        //println!("{:?}", extracts_path);
        fs::create_dir_all(&extracts_path).expect("Unable to create extraction path");

        extracts_path.push(file_name);

        extracts_path

    }

    ///extracts an arcive into a list of ExtractedFile types.
    /// An Extracted file can be one of two variants. Csv or ImageArchive
    ///
    ///
    ///
    pub fn extract_archive(&mut self, sx_archive: &SexOffenderArchive) -> Result<Vec<ExtractedFile>> {
        let mut extracted_files: Vec<ExtractedFile> = Vec::new(); //store our list of csv files.
        let mut archive_path: &PathBuf = &sx_archive.path; //this could come from the config. think on that
        let mut state_abbrev = &archive_path.file_name().unwrap().to_str().unwrap()[..2];
        let file = BufReader::new(File::open(&archive_path)?);
        let mut archive = zip::ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut embedded_file = archive.by_index(i)?;

      /*      if state_abbrev == "TX" { //Texas is a problem. Lots of files, all fucked.
                continue;
            }
*/
            let embedded_file_name = embedded_file.sanitized_name();
            let extracts_path = self.generate_extract_path(&state_abbrev, &archive_path, embedded_file_name.as_os_str());
            let mut outfile = BufWriter::new(File::create(&extracts_path)?);
            std::io::copy(&mut embedded_file, &mut outfile)?;
            //println!("wrote: {}", extracts_path.display());

            match embedded_file_name.extension() {
                Some(ext) if ext == "csv" => {
                    let ex_file = ExtractedFile::Csv { path: extracts_path, state: String::from(state_abbrev), delimiter: '|' };
                    println!("csv file: {:?}", &ex_file);

                    extracted_files.push(ex_file);
                },
                //TODO: make sure that files with txt extension always have a "," as delimiter. We're assuming based on incomplete data
                Some(ext) if ext == "txt" => {
                    let ex_file = ExtractedFile::Csv { path: extracts_path, state: String::from(state_abbrev), delimiter: ',' };
                    println!("txt file: {:?}", &ex_file);
                    extracted_files.push(ex_file);
                }
                Some(ext) if ext == "zip" => {
                    let ex_file = ExtractedFile::ImageArchive { path: extracts_path, state: String::from(state_abbrev) };
                    extracted_files.push(ex_file);
                }
                None => {
                    println!("No file extension found in extracted file!");
                },
                _ => {
                    println!("Unsupported file extension found and ignored.");
                }
            }
        }
        Ok(extracted_files)
    }
}
