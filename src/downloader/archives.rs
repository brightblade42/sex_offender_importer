/* THIS FILE NOT IN USE */
use std::path::{PathBuf};
use serde::{Deserialize, Serialize};

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
