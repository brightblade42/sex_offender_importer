use crate::types::{SexOffenderArchive, ExtractedFile};
use std::path::{Path, PathBuf};
use std::io::{BufWriter, BufReader};
use std::fs::{self, File};
use std::ffi::OsStr;


type GenError = Box<dyn std::error::Error>;
pub type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Extractor {

}

impl Extractor {

    fn generate_extract_path(&self, state: &str, archive_path: &PathBuf, file_name: &OsStr) -> PathBuf {
        let mut extraction_path = archive_path.parent().unwrap().to_path_buf().clone();
        extraction_path.push(state);

        if archive_path.to_string_lossy().contains("records") {
            extraction_path.push("records");
        } else {
            extraction_path.push("images");
        }

        println!("{:?}", extraction_path);
        fs::create_dir_all(&extraction_path).expect("Unable to create extraction path");

        extraction_path.push(file_name);

        extraction_path

    }

    ///extracts an arcive into a list of ExtractedFile types.
    /// An Extracted file can be one of two variants. Csv or ImageArchive
    ///
    ///
    ///
    pub fn extract_archive(&mut self, sx_archive: SexOffenderArchive) -> Result<Vec<ExtractedFile>> {
        let mut extracted_files: Vec<ExtractedFile> = Vec::new(); //store our list of csv files.
        let mut archive_path: PathBuf = sx_archive.path;
        let mut state_abbrev = &archive_path.file_name().unwrap().to_str().unwrap()[..2];
        let file = BufReader::new(File::open(&archive_path)?);
        let mut archive = zip::ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut embedded_file = archive.by_index(i)?;

            if state_abbrev == "TX" { //Texas is a problem. Lots of files, all fucked.
                continue;
            }

            let file_name = embedded_file.sanitized_name();
            let extraction_path = self.generate_extract_path(state_abbrev.clone(), &archive_path, file_name.as_os_str());
            let mut outfile = BufWriter::new(File::create(&extraction_path)?);
            std::io::copy(&mut embedded_file, &mut outfile)?;
            println!("wrote: {}", extraction_path.display());

            match file_name.extension() {
                Some(ext) if ext == "csv" => {
                    let ex_file = ExtractedFile::Csv { path: extraction_path.clone(), state: String::from(state_abbrev), delimiter: '|' };
                    println!("csv file: {:?}", &ex_file);

                    extracted_files.push(ex_file);
                },
                //TODO: make sure that files with txt extension always have a "," as delimiter. We're assuming based on incomplete data
                Some(ext) if ext == "txt" => {
                    let ex_file = ExtractedFile::Csv { path: extraction_path.clone(), state: String::from(state_abbrev), delimiter: ',' };
                    println!("txt file: {:?}", &ex_file);
                    extracted_files.push(ex_file);
                }
                Some(ext) if ext == "zip" => {
                    let ex_file = ExtractedFile::ImageArchive { path: extraction_path.clone(), state: String::from(state_abbrev) };
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
