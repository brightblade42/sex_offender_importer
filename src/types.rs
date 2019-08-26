use std::fs::{self,File};
use std::error::Error;
use std::path::{self, PathBuf};
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

pub struct Csv {
    pub path: PathBuf,
    pub state: String,
    pub delimiter: char,
}

pub struct ImageArchive {
    pub path: PathBuf,
    pub state: String,
}

trait Import {
    fn prepare_for_import(&self);
    fn add_headers_to_file(&self) -> Result<(),dyn err::Error>;
    fn load_csv_info(&self) -> Option<Vec<CsvMetaData>>;
}

impl Import for Csv {
    fn prepare_for_import(&self) {}
    fn add_headers_to_file(&self) -> Result<(), dyn err::Error> {

        let csv_meta = self.load_csv_info();

        let delim = &self.delimiter;
        csv_meta.iter().for_each(|md| {

            let mut tab_list = String::new();

            let mut tlist: String = tf.headers.iter().fold(tab_list, |acc, head | {
                format!("{}{}{}",acc, &delim, head )
            });

            tlist.push_str("\n");
            let tlist = tlist.trim_start();

            println!("Looks like we made it!");

        });

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

impl Import for ExtractedFile {

    fn prepare_for_import(&self) {

        match self {
            ExtractedFile::Csv(csv) => {

            },
            _ => None
        }
    }

    fn add_headers_to_file(&self) -> Result<(), dyn err::Error> {

        match self {
            ExtractedFile::Csv(csv) => {
                csv.add_headers_to_file()
            },
            _ => Ok(()) //just passin through
        }

    }

    fn load_csv_info(&self) -> Option<Vec<CsvMetaData>> {

        match self {
            ExtractedFile::Csv(csv) => {
                csv.load_csv_info()
            },
            _ => None
        }
    }
}
