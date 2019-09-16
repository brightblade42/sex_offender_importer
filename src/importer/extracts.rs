use std::fs::{self,File};
use std::error::Error;
use std::path::{self, PathBuf, Path};
use std::borrow::{Borrow};
use std::{io, io::Write};

use mktemp::Temp;
use serde;
use serde_derive::{Serialize, Deserialize};
use rusqlite::{Connection, NO_PARAMS, params};
use bytes::Buf;
use std::io::BufReader;
use super::{Import, SqlHandler};

///CsvMetaData contains some basic information about a csv file.
///name: The file name and extension of the csv file.
///headers: A vector of headers that should be associated with the csv file
///Currently headers is used in situations where the csv file doesn't include headers
///Headers are required for importing. This allows us to add some to a csv file if they don't exist
#[derive(Debug, Serialize)]
pub struct CsvMetaData<'a> {
    name: String,
    headers: Vec<&'a str>,
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

        if let Some(meta) = self.load_csv_info() {
                meta.iter().for_each(|md| {
                    if self.path.ends_with(md.name.clone()) {
                        let mut header_line = self.build_header_line(md);
                        self.prepend_file(header_line.as_bytes(), &self.path);
                        println!("{}", &header_line);
                    }
                });
        }

        Ok(())

    }

    ///transform vec of headers to a delimitted text string
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

}


impl Import for Csv {
    type Reader = csv::Reader<File>;

    fn open_reader(&self, has_headers: bool) -> Result<Self::Reader , Box<dyn Error>> {

        let file = File::open(&self.path)?; //.unwrap();

        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(self.delimiter as u8)
            .trim(csv::Trim::All)
            .has_headers(has_headers)
            //  .flexible(true)
            .from_reader(file);

        Ok(rdr)
    }

    fn import(&self) -> Result<(), Box<dyn Error>>  {
        self.add_headers();
//        self.import_file_data()
        Ok(())
    }

    fn import_file_data(&self, conn: &Connection) -> Result<(), Box<dyn Error>> {

        //this should be filtered out, really.
        let dsp = self.path.display().to_string();
        //TODO: filter this out from elsewhere methinks.
        if dsp.contains("screenshot") {
            println!("skipping screenshot file. It is useless");
            return Ok(());
        }

        let has_headers = true;
        let mut csv_reader = self.open_reader(has_headers)?;

        let table_name = String::from(self.path.file_stem().unwrap().to_str().unwrap());
        println!("=============================");
        println!("Dropped: {}", &table_name);
        self.drop_table(&conn, &table_name)?;

        let mut table_query = self.create_table_query(Some(&mut csv_reader), &table_name)?;

        println!("Creating {}", &table_name);
        println!("=============================");
        println!("{}", &table_query);
        println!("=============================");
        conn.execute(&table_query, NO_PARAMS)?;

        //TODO: create default index
        //create default index
        //let defindex = create_default_index(&table_name);
        // conn.execute(&defindex, NO_PARAMS);

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
                    rec.push_field(self.state.as_bytes());
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



impl SqlHandler for Csv {
    type Reader = csv::Reader<File>;
    fn create_table_query(&self, reader: Option<&mut Self::Reader>, tname: &str) -> Result<String, Box<dyn Error>> {

        let reader = reader.unwrap();
        let  convert_space_in_field = | field: String| -> String {

            if field.trim().contains(" ") {
                field.replace(" ", "_")

            } else {
                String::from(field)
            }
        };

        let  convert_state_field = |field: &str|  -> String {

            if field == "State" || field == "state" {
                String::from("Addr_State")
            } else {
                String::from(field)
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
    fn create_insert_query(&self, reader: &mut Self::Reader, tname: &str) -> Result<String, Box<dyn Error>> {

        Ok("".to_string())
    }
    fn execute(&self, conn: &Connection) -> Result<usize, rusqlite::Error> {
        Ok(0)
    }
}

