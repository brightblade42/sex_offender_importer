//!
//!
//!
//!
//!
//!
//!
//!
use std::{
    fs::{self,File},
    path::{PathBuf, Path},
    io::{self, Write},
};

use super::{Import, SqlHandler, set_pragmas};
use crate::util::{
    self,
    GenResult,
};
use mktemp::Temp;
use serde_derive::{Serialize, Deserialize};
use rusqlite::{Connection, NO_PARAMS};
use std::error::Error;
use std::fs::DirEntry;

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


///represents a csv file
#[derive(Debug, Serialize, Deserialize)]
pub struct Csv {
    ///path to csv file on disk
    pub path: PathBuf,
    ///the US state to which this csv file belongs
    pub state: String,
    ///the csv delimiter used to separate columns. not all files actually use a comma
    ///despite the name  `comma separated values`
    pub delimiter: char,
}

impl Csv {

    ///Some csv files may not have headers, in those cases we want to add some.
    ///We need headers to import into the database. Headers represent column names
    fn add_headers(&self) -> GenResult<()> {

        if let Some(meta) = self.load_csv_info() {
                meta.iter().for_each(|md| {
                    if self.path.ends_with(md.name.clone()) {
                        let header_line = self.build_header_line(md);
                        self.add_headers_to_file(header_line.as_bytes(), &self.path).unwrap();
                        println!("{}", &header_line);
                    }
                });
        }

        Ok(())

    }

    ///transform vec of headers to a delimitted text string
    fn build_header_line(&self, csv_meta: &CsvMetaData) -> String {

        let header_string = String::new();
        let mut header_line: String = csv_meta.headers.iter().fold(header_string, |acc, head | {
            format!("{}{}{}",acc, self.delimiter, head )
        });

        header_line.push_str("\n");

        String::from(header_line.trim_start())
    }


    ///Adds a header line to the top of a csv file that doesn't contain any headers.
    ///
    ///The import process requires that all csv files have headders, most do but some, don't.
    ///looking at you Texas.
    ///This modifies existing csv files on disk
    fn add_headers_to_file(&self, data: &[u8], file_path: &Path) -> io::Result<()> {
        //Temporary file dancing and shenanigans.
        println!("ex path: {:?}", file_path );
        let tmp_path = Temp::new_file()?;

        // Stop the temp file being automatically deleted when the variable
        // is dropped, by releasing it.
        let tmp_path = tmp_path.release();
        let mut tmp_file = File::create(&tmp_path)?;
        // Open the source file for reading
        let mut src_file = File::open(&file_path)?;
        // Write the data to prepend
        tmp_file.write_all(&data)?;
        // Copy the rest of the source file
        io::copy(&mut src_file, &mut tmp_file)?;
        fs::remove_file(&file_path)?;
        fs::copy(&tmp_path, &file_path)?;
        fs::remove_file(&tmp_path)?;
        Ok(())
    }

    ///
    fn load_csv_info(&self) -> Option<Vec<CsvMetaData>> {

        //Texas is the only state providing csv files without headers. It's a bummer.
        //I've gleaned the necessary header information from the SQLSchema dump files provided
        //directly from the Texas web site.
        //TODO: keep an eye on this, it's entirely possible that these could change based on the whims
        //of Texas.
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
                    //headers: vec!["COO_COD","COJ_COD","JOO_COD","OFF_COD","VER_NBR","LEN_TXT","STS_COD","CIT_TXT"], //,"BeginDate","EndDate"]
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
                    name: "INDV.txt".into(),
                    headers: vec!["IND_IDN","DPS_NBR"]
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


    ///After csv data is converted to table data, we still need to convert data
    /// into the final data format which goes into the main SexOffender table.
    fn import_final_phase(&self) -> GenResult<()> {
        let mut pth = PathBuf::from(util::SQL_FOLDER);
        pth.push(format!("{}_import.sql", self.state.to_lowercase()));

        if !pth.exists() {
            println!("The import file is missing: {}", &pth.display());
            return Ok(());
        }

        let final_import_query = fs::read_to_string(pth)?;

        println!("=======================================" );
        println!("{}", &final_import_query);
        println!("=======================================" );
        let conn = util::get_connection(None)?;
        //conn.execute(&format!("Delete from SexOffender where state='{}'", self.state), NO_PARAMS)?;
        conn.execute(&final_import_query, NO_PARAMS).expect("Unable to do final import");

        Ok(())
    }


    fn find_date_position(&self, headers: &csv::StringRecord) -> Option<usize> {

        match self.state.as_str() {
            "NJ" => {
                let date_header = "age";
                let mut dt = None;
                for (i, hdr) in headers.iter().enumerate() {
                       if hdr == date_header {
                           dt = Some(i);
                       }
                }
                dt
            },
            _ => None
        }
    }
}


impl Import for Csv {
    type Reader = csv::Reader<File>;

    fn open_reader(&self, has_headers: bool) -> GenResult<Self::Reader> {

        let file = File::open(&self.path)?; //.unwrap();

        let rdr = csv::ReaderBuilder::new()
            .delimiter(self.delimiter as u8)
            .trim(csv::Trim::All)
            .has_headers(has_headers)
            //  .flexible(true)
            .from_reader(file);

        Ok(rdr)
    }


    fn import(&self) -> GenResult<()>  {
        self.add_headers()?;
        self.import_file_data()?;
//        self.import_final_phase()?;
        Ok(())
    }

    fn import_file_data(&self) -> GenResult<()> {

        let conn = util::get_connection(None).expect("Unable to connect to db");

        set_pragmas(&conn);
        //this should be filtered out, really.
        let dsp = self.path.display().to_string();
        //TODO: filter this out from elsewhere methinks.
        if dsp.contains("screenshot") {
            println!("skipping screenshot file. It is useless");
            return Ok(());
        }

        let has_headers = true;
        let mut csv_reader = self.open_reader(has_headers)?;
        let mut  table_name = String::from(self.path.file_stem().unwrap().to_str().unwrap());
        if self.state == "TX" {
           table_name = format!("TX{}", table_name);
        }

        //TODO:: ensure this is temporary.
        let create_query = self.create_table_query(&mut csv_reader, &table_name)?;
        conn.execute(&create_query, NO_PARAMS)?;

        self.delete_data(&conn, &table_name)?;
        let insert_query = self.create_insert_query(&mut csv_reader, &table_name)?;


        conn.execute("Begin Transaction;", NO_PARAMS)?;

        let mut insStmt = conn.prepare(&insert_query);
        let date_pos = self.find_date_position(&csv_reader.headers().unwrap());
        let mut rec_vals: Vec<String> = Vec::new();
        //we use bytes to work around non-utf-8 issues in csv.
        for result in csv_reader.byte_records() {
            match result {
                Ok(record) => {
                    let rec = record.clone();

                    ///TODO:: Extract this to a function, this will grow
                    /// as more formatting is needed
                    for (idx, rr) in rec.iter().enumerate() {
                        let mut ascii_string = util::to_ascii_string(&rr);

                        if let Some(i) = date_pos {
                           if i == idx {
                               ascii_string = util::format_date(&ascii_string).to_string();
                           }
                        }

                        rec_vals.push(ascii_string.parse().unwrap());
                    }

                    rec_vals.push(self.state.to_uppercase());

                   match &insStmt.as_mut().unwrap().execute(&rec_vals) {
                   // match conn.execute(&insert_query, &rec_vals)  {
                        Ok(_) =>  () ,
                        Err(er) => {
                            //TODO: Consider logging these kinds of errors.
                            println!("Unable to insert csv record: {}", er.description());
                           println!("csv record: {:?}", rec_vals );
                        }
                    }
                    rec_vals.clear();
                }
                Err(e) => {

                    println!("Row data error: {}", e);
                    println!("vals {:?}", rec_vals );
                }
            }
        }

        conn.execute("COMMIT TRANSACTION;", NO_PARAMS)?;
        Ok(())
    }
}



impl SqlHandler for Csv {

    type Reader = csv::Reader<File>;
    fn create_table_query(&self, reader: &mut Self::Reader, tname: &str) -> GenResult<String> {


        let q = format!("CREATE TABLE if not exists {} (", tname);
        //add the header names as column names for out table.
        let mut create_table = reader
            .headers()
            .unwrap()
            .iter()
            .map(util::convert_state_field)
            .map(util::convert_invalid_field_name)
            .map(util::convert_space_in_field)
            .fold(q, |acc, head| {
                format!("{} {},", acc, head.replace("/", ""))
            });

        create_table.push_str("state )"); //add our extra state field and close the ()

        Ok(create_table)
    }

    fn create_insert_query(&self, reader: &mut Self::Reader, tname: &str) -> GenResult<String> {

        //being constructing the insert statement.
        let q = format!("INSERT INTO {} ( ", tname);

        //add the headers as columns
        let mut insert_query = reader
            .headers()
            .unwrap()
            .iter()
            .map(util::convert_state_field)
            .map(util::convert_invalid_field_name)
            .map(util::convert_space_in_field)
            .fold(q, |acc, head| format!("{} {},", acc, head.replace("/", "")));

        insert_query.push_str("state ) VALUES ("); //add our extra state field and close the ()

        //add value parameter placeholders
        //TODO: this seems like it could be better.
        let header_count = reader.headers().unwrap().len();

        for _i in 0..header_count {
            insert_query.push_str("?,");
        }

        insert_query.push_str("?)"); //our state parameter.

       println!("Query: {}", &insert_query);

        Ok(insert_query)
    }

    //this is not currently used
    fn execute(&self, _conn: &Connection) -> Result<usize, rusqlite::Error> {
        Ok(0)
    }
}

