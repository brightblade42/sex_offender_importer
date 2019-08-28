use std::fs::{self,File};
use std::error::Error;
use std::path::{self, PathBuf, Path};
use std::borrow::{Borrow};
use std::{io, io::Write};

use mktemp::Temp;
use serde;
use serde_derive::{Serialize, Deserialize};
use rusqlite::{Connection, NO_PARAMS};
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

///CsvMetaData contains some basic information about a csv file.
///name: The file name and extension of the csv file.
///headers: A vector of headers that should be associated with the csv file
///Currently headers is used in situations where the csv file doesn't include headers
///Headers are required for importing. This allows us to add some to a csv file if they don't exist
#[derive(Debug, Serialize)]
struct CsvMetaData<'a> {
    name: String,
    headers: Vec<&'a str>,
}

///ExtractedFile represents one of two possible types that are contained within
/// the zip archives downloaded from the remote server.
#[derive(Debug, Serialize, Deserialize)]
pub enum ExtractedFile {
    Csv(Csv),
    ImageArchive(ImageArchive)
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Csv {
    pub path: PathBuf,
    pub state: String,
    pub delimiter: char,
}

impl Csv {

    //Some csv files may not have headers, in those cases we want to add some.
    fn add_headers(&self) -> Result<(), Box<dyn Error>> {

        let csv_meta = self.load_csv_info();

        csv_meta.unwrap().iter().for_each(|md| {
            if self.path.ends_with(md.name.clone()) {
                let mut header_line = self.build_header_line(md);
                self.prepend_file(header_line.as_bytes(), &self.path);
                println!("{}", &header_line);
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


    //Does the actual adding of the header line to a csv file.
    fn prepend_file(&self, data: &[u8], file_path: &Path) -> io::Result<()> {
        // Create a temporary file
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

        //Texas is the only state providing csv files without headers. It's a bummer.
        //I've gleaned the necessary header information from the SQLSchema dump files provided
        //directly from the Texas web site.
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
                    headers: vec!["COO_COD","COJ_COD","JOO_COD","OFF_COD","VER_NBR","LEN_TXT","STS_COD","CIT_TXT"], //,"BeginDate","EndDate"]
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

    /*
    fn ToSql(&self) -> (String, String) {

       let name = self.path.components().last().expect("Unable to get fiename component");
       let tbl_name =  String::from(self.state).push_str(name.to_owned().as_os_str().to_str().unwrap());
       let idx_stmt =  format!(r#"CREATE INDEX if not exists {}__index ON {} (ID, State);"#, &tbl_name);
       let ins_stmt = format!(r#""#)


        (String::from(""), String::from(""))
    }*/
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageArchive {
    pub path: PathBuf,
    pub state: String,
}

trait SqlHandler {

    fn create_table_query(&self, reader: &mut csv::Reader<File>, tname: &str) -> Result<String, Box<dyn Error>>;
    fn create_insert_query(&self,reader: &mut csv::Reader<File>, tname: &str) -> Result<String, Box<dyn Error>>;
    fn drop_table(&self, conn: &Connection, table_name: &str) -> Result<usize, rusqlite::Error> {
        let r = conn.execute(&format!("DROP TABLE if exists {}", table_name), NO_PARAMS);
        r
    }

    fn execute(&self, conn: &Connection) -> Result<usize, rusqlite::Error>;

}

impl SqlHandler for Csv {

    fn create_table_query(&self, reader: &mut csv::Reader<File>, tname: &str) -> Result<String, Box<dyn Error>> {

        let convert_space_in_field = |field: &str| -> &str {

            if field.trim().contains(" ") {
                field.replace(" ", "_").as_str()
            } else {
                field
            }
        };

        let convert_state_field = |field: &str|  -> &str{

            if field == "State" || field == "state" {
                "Addr_State"
            } else {
                field
            }
        };


        let mut q = format!("CREATE TABLE if not exists {} (", tname);
        //add the header names as column names for out table.
        let mut create_table = reader
            .headers()
            .unwrap()
            .iter()
            .map(convert_state_field)
            .map(convert_space_in_field)
            .fold(q, |acc, head| {
                format!("{} {},", acc, head.replace("/", ""))
            });

        create_table.push_str("state )"); //add our extra state field and close the ()

        Ok(create_table)
    }
    fn create_insert_query(&self, reader: &mut csv::Reader<File>, tname: &str) -> Result<String, Box<dyn Error>> {

        Ok("".to_string())
    }
    fn execute(&self, conn: &Connection) -> Result<usize, rusqlite::Error> {
        Ok(0)
    }
}


pub trait Import {
    type Reader;
    fn open_reader() -> Result<Self::Reader , Box<dyn Error>> {

        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(delim as u8)
            .trim(csv::Trim::All)
            .has_headers(has_headers)
            //  .flexible(true)
            .from_reader(file);

        Ok(rdr)
    }


    fn import(&self) -> Result<(), Box<dyn Error>>;
    fn import_csv_files(&self, conn: &Connection) -> Result<(), Box<dyn Error>>;
}

impl Import for Csv {
    type Reader = csv::Reader<File>;

    fn import(&self) -> Result<(), Box<dyn Error>>  {
        self.add_headers();
       Ok(())
    }

    fn import_csv_files(&self, conn: &Connection) -> Result<(), Box<dyn Error>> {

        //this should be filtered out, really.
        let dsp = self.path.display().to_string();
        //TODO: filter this out from elsewhere methinks.
        if dsp.contains("screenshot") {
            println!("skipping screenshot file. It is useless");
            return Ok(());
        }

        let file = File::open(&self.path)?; //.unwrap();

        // let has_headers = if state == "TX" { false } else { true };
        //how to decide on a delimiter
        let mut csv_reader = open_csv_reader(file, delimiter.to_owned(), true)?;

        let table_name = String::from(path.file_stem().unwrap().to_str().unwrap());
        println!("=============================");
        println!("Dropped: {}", &table_name);
        drop_table(&conn, &table_name)?;

        let mut table_query = self.create_table_query(&mut csv_reader, &table_name)?;

        println!("Creating {}", &table_name);
        println!("=============================");
        println!("{}", &table_query);
        println!("=============================");
        conn.execute(&table_query, NO_PARAMS)?;

        //create default index
        let defindex = create_default_index(&table_name);
        conn.execute(&defindex, NO_PARAMS);

        let insert_query = self.create_insert_query(&mut csv_reader, &table_name)?;

        println!("=============================");
        println!("{}", &insert_query);

        println!("=============================");

        conn.execute("Begin Transaction;", NO_PARAMS);

        //insert a record (line of csv) into sqlite table.
        //we use as_bytes() because some data is not utf-8 compliant
        let mut sql_err = 0;
        for result in csv_reader.byte_records() {
            match result {
                Ok(record) => {
                    let mut rec = record.clone();
                    if table_name == "OFF_CODE_SOR" {
                        println!("{}",rec.len());
                    }
                    rec.push_field(state.as_bytes());
                    let res = conn.execute(&insert_query, &rec).expect("A good insert");
                }
                Err(e) => {
                    println!("Row data error: {}", e);
                }
            }

        }
        conn.execute("COMMIT TRANSACTION;", NO_PARAMS);
        Ok(())
    }
}



impl Import for ImageArchive {
    fn import(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn import_csv_files(&self, conn: &Connection) -> Result<(), Box<dyn Error>> {
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

    fn import_csv_files(&self, conn: &Connection) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
