use crate::data::types::DataSqlType;
use linked_hash_map::LinkedHashMap;
use serde::Deserialize;
use serde_json::Value;
use std::{fs::File, io::Read};

#[derive(Deserialize, Debug, Clone)]
pub struct DataColumnModel {
    pub name: String,
    pub r#type: DataSqlType,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DataInsertModel {
    pub columns: Vec<DataColumnModel>,
    pub inserts: Vec<LinkedHashMap<String, Value>>,
}

#[derive(Deserialize, Debug)]
pub struct RawDataModel {
    pub data: LinkedHashMap<String, DataInsertModel>,
}

#[derive(Debug)]
pub enum ParserError {
    OpenFileError,
    ReadFileError,
    ParseJsonError,
}

pub fn from_file(path: &str) -> Result<RawDataModel, ParserError> {
    match File::open(path) {
        Ok(mut file) => {
            let mut data = String::new();

            if let Err(_) = file.read_to_string(&mut data) {
                return Err(ParserError::ReadFileError);
            }

            match serde_json::from_str::<RawDataModel>(&mut data) {
                Ok(json) => Ok(json),
                Err(a) => {
                    println!("{:?}", a);
                    return Err(ParserError::ParseJsonError);
                }
            }
        }
        Err(_) => return Err(ParserError::OpenFileError),
    }
}

#[cfg(test)]
mod data_tests {
    use core::panic;

    use crate::data::{self, types};

    #[test]
    fn test_from_file() {
        match data::parser::from_file("./testdata/data-test.json") {
            Ok(json) => match json.data.get("user") {
                Some(user) => {
                    assert_eq!("id", user.columns[0].name);
                    assert_eq!(types::DataSqlType::UUID, user.columns[0].r#type);
                    assert_eq!("username", user.columns[1].name);
                    assert_eq!("is_student", user.columns[2].name);
                }
                None => {
                    panic!("Invalid key");
                }
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}
