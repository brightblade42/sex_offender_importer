use std::fs::{self,File};
use std::error::Error;
use std::path::{self, PathBuf, Path};
use std::borrow::{Borrow};
use std::{io, io::Write};

use mktemp::Temp;
use serde;
use serde_derive::{Serialize, Deserialize};
//use crate::types::ExtractedFile::{Csv, ImageArchive};

static SEX_OFFENDER_PATH: &'static str = "";

#[derive(Debug, Serialize, Deserialize)]
pub struct SexOffenderArchive {
    pub path: PathBuf,
    pub size: usize,
}

impl SexOffenderArchive {
    pub fn new(path: PathBuf, size: usize) -> Self {
        Self {
            path,
            size,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OffenderArchive {
    Record(SexOffenderArchive),
    Image(SexOffenderArchive)
}



#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum RecordStatus {
    None,
    InFlight,
    Failed,
    Downloaded,
}


#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct RecordInfo {
    pub rpath: Option<String>,
    pub name: Option<String>,
    pub last_modified: Option<String>,
    pub size: Option<i64>,
    //convert this to i64
    pub status: RecordStatus,
}


#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct ImageInfo {
    pub rpath: Option<String>,
    pub name: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
#[serde(tag = "type")]
pub enum FileInfo {
    Record(RecordInfo),
    Image(ImageInfo),
}

impl FileInfo {
    pub fn name(&self) -> String {
        use FileInfo::*;
        match *self {
            Record(ref r) => r.name.as_ref().unwrap().to_string(),
            Image(ref i) => i.name.as_ref().unwrap().to_string(), //as_ref().unwrap()
        }
    }

    pub fn base_path(info: &FileInfo) -> PathBuf {
        PathBuf::from("/some/cool/path")
    }

    pub fn remote_path(&self) -> path::PathBuf {
        use FileInfo::*;

        path::PathBuf::from(
            match *self {
                Record(ref r) => format!("/{}{}", r.rpath.as_ref().unwrap(), SEX_OFFENDER_PATH),
                Image(ref i) => format!("/{}{}", i.rpath.as_ref().unwrap(), SEX_OFFENDER_PATH),
            }
        )
    }
}


#[derive(Debug, Serialize)]
struct CsvMetaData<'a> {
    name: String,
    headers: Vec<&'a str>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum ExtractedFile {
    Csv(Csv),
    ImageArchive(ImageArchive)
    //Csv { path: PathBuf, state: String, delimiter: char },
    //ImageArchive { path: PathBuf, state: String },
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Csv {
    pub path: PathBuf,
    pub state: String,
    pub delimiter: char,
}

impl Csv {

    fn add_headers(&self) -> Result<(), Box<dyn Error>> {

        let csv_meta = self.load_csv_info();

      //  let matched_file = csv_meta.unwrap().iter().find(|f| self.path.ends_with(f.name));
        csv_meta.unwrap().iter().for_each(|md| {
            if self.path.ends_with(md.name.clone()) {
                let mut header_line = self.build_header_line(md);
                self.prepend_file(header_line.as_bytes(), &self.path);
                println!("Looks like we made it!");
            }
        });

        Ok(())

    }

    fn build_header_line(&self, csv_meta: &CsvMetaData) -> String {

        let mut header_string = String::new();
        let mut header_line: String = csv_meta.headers.iter().fold(header_string, |acc, head | {
            format!("{}{}{}",acc, self.delimiter, head )
        });

        header_line.push_str("\n");

        String::from(header_line.trim_start())
    }


    fn prepend_file(&self, data: &[u8], file_path: &Path) -> io::Result<()> {
        // Create a temporary file
        //let fp = PathBuf::from(file_path.clone();
        println!("ex path: {:?}", file_path );
        let mut tmp_path = Temp::new_file()?;

        // Stop the temp file being automatically deleted when the variable
        // is dropped, by releasing it.
        let tmp_path = tmp_path.release();

        let mut tmp_file = File::create(&tmp_path)?;
        // Open source file for reading
        let mut src_file = File::open(&file_path)?;
        // Write the data to prepend
        tmp_file.write_all(&data)?;
        // Copy the rest of the source file
        io::copy(&mut src_file, &mut tmp_file)?;
        fs::remove_file(&file_path)?;
        fs::copy(&tmp_path, &file_path);
        fs::remove_file(&tmp_path);
        Ok(())
    }

    fn load_csv_info(&self) -> Option<Vec<CsvMetaData>> {

        if self.state == "TX" {
            Some(vec![
                CsvMetaData {
                    name: "Address.txt".into(),
                    headers: vec!["AddressId", "IND_IDN", "SNU_NBR", "SNA_TXT", "SUD_COD", "SUD_NBR", "CTY_TXT", "PLC_COD", "ZIP_TXT", "COU_COD", "LAT_NBR", "LON_NBR"],
                },
                CsvMetaData {
                    name: "BRTHDATE.txt".into(),
                    headers: vec!["DOB_IDN","PER_IDN","TYP_COD","DOB_DTE"]
                },
                CsvMetaData {
                    name: "NAME.txt".into(),
                    headers: vec!["NAM_IDN","PER_IDN","TYP_COD","NAM_TXT","LNA_TXT","FNA_TXT"]
                },
                CsvMetaData {
                    name: "OFF_CODE_SOR.txt".into(),
                    headers: vec!["COO_COD","COJ_COD","JOO_COD","OFF_COD","VER_NBR","LEN_TXT","STS_COD","CIT_TXT","BeginDate","EndDate"]
                },
                CsvMetaData {
                    name: "Offense.txt".into(),
                    headers: vec!["IND_IDN","OffenseId","COO_COD","COJ_COD","JOO_COD","OFF_COD","VER_NBR","GOC_COD","DIS_FLG","OST_COD","CPR_COD","CDD_DTE","AOV_NBR","SOV_COD","CPR_VAL"]

                },
                CsvMetaData {
                    name: "Photo.txt".into(),
                    headers: vec!["IND_IDN","PhotoId","POS_DTE"]
                },
                CsvMetaData {
                    name: "PERSON.txt".into(),
                    headers: vec!["IND_IDN","PER_IDN","SEX_COD","RAC_COD","HGT_QTY","WGT_QTY","HAI_COD","EYE_COD","ETH_COD"]
                },
            ])
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageArchive {
    pub path: PathBuf,
    pub state: String,
}

pub trait Import {
    fn import(&self) -> Result<(), Box<dyn Error>>;
}

impl Import for Csv {
    fn import(&self) -> Result<(), Box<dyn Error>>  {
        self.add_headers();
       Ok(())
    }
}
impl Import for ImageArchive {
    fn import(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl Import for ExtractedFile {

    fn import(&self) -> Result<(), Box<dyn Error>> {
        match self {

            ExtractedFile::Csv(csv) => {
                csv.import()
            },
            _ => Ok(())
        }
    }
}
