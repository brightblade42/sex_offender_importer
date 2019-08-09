use std::fs;
use std::path::{self, PathBuf};
use serde;
use serde_derive::{Serialize, Deserialize};

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
pub enum ExtractedFile {
    Csv { path: PathBuf, state: String, delimiter: char },
    ImageArchive { path: PathBuf, state: String },
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
