use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

use data::parser::ParserError;
use json::{JsonDataArray, JsonRawModel};
use sqlizer::StringifyInsertError;

mod data;
mod json;
mod mapper;
mod sqlizer;

pub struct GenInsertOptions<'a> {
    pub split_data_files: bool,
    pub sql_out_dir_path: &'a str,
    pub sql_out_file_path: &'a str,
    pub json_out_dir_path: &'a str,
    pub json_out_file_path: &'a str,
}

impl<'a> Default for GenInsertOptions<'a> {
    fn default() -> GenInsertOptions<'a> {
        GenInsertOptions {
            split_data_files: false,
            sql_out_dir_path: "./out",
            json_out_dir_path: "./out/json",
            sql_out_file_path: "./out.sql",
            json_out_file_path: "./out/out.json",
        }
    }
}

#[derive(Debug)]
pub enum GenInsertsFromFileError<'a> {
    SqlOutFileWriteError(ForceWriteAllError),
    StringifyError(StringifyInsertError),
    ParserError(ParserError),
    OutDirRequired(&'a str),
    JsonOutFileWriteError(ForceWriteAllError),
    JsonSerializeError,
}

pub fn gen_inserts_from_file<'a>(
    path: &str,
    opts: GenInsertOptions,
) -> Result<(), GenInsertsFromFileError<'a>> {
    match data::parser::from_file(path) {
        Ok(model) => {
            let sql_models = mapper::map_data_to_inserts(&model);
            let json_models = mapper::map_data_to_json(&model);

            if opts.split_data_files {
                // map to sql files
                for sm in sql_models.iter() {
                    match sqlizer::sqlize_insert(sm) {
                        Ok(sql) => {
                            let insert_str = format!("{}\n\n", sql.to_owned());
                            let file_path =
                                format!("{}/{}.sql", opts.sql_out_dir_path, sm.table_name);
                            if let Err(e) = force_write_all(&file_path, insert_str.as_bytes()) {
                                return Err(GenInsertsFromFileError::SqlOutFileWriteError(e));
                            }
                        }
                        Err(e) => {
                            return Err(GenInsertsFromFileError::StringifyError(e));
                        }
                    }
                }

                // map to json files
                for (name, insert_model) in model.data.iter() {
                    let json = mapper::map_insert_to_json(name, insert_model);

                    let json = match serde_json::to_string::<HashMap<String, JsonDataArray>>(&json)
                    {
                        Ok(j) => j,
                        Err(_) => {
                            return Err(GenInsertsFromFileError::JsonSerializeError);
                        }
                    };

                    let file_path = format!("{}/{}.json", opts.json_out_dir_path, name);
                    if let Err(e) = force_write_all(&file_path, json.as_bytes()) {
                        return Err(GenInsertsFromFileError::JsonOutFileWriteError(e));
                    }
                }

                return Ok(());
            }

            // one file
            let sql_str = match sqlizer::compose_inserts(sql_models) {
                Ok(sql) => sql,
                Err(e) => return Err(GenInsertsFromFileError::StringifyError(e)),
            };

            if let Err(e) = force_write_all(opts.sql_out_file_path, sql_str.as_bytes()) {
                return Err(GenInsertsFromFileError::SqlOutFileWriteError(e));
            }

            let json = match serde_json::to_string::<JsonRawModel>(&json_models) {
                Ok(j) => j,
                Err(_) => {
                    return Err(GenInsertsFromFileError::JsonSerializeError);
                }
            };

            if let Err(e) = force_write_all(opts.json_out_file_path, json.as_bytes()) {
                return Err(GenInsertsFromFileError::SqlOutFileWriteError(e));
            }

            return Ok(());
        }
        Err(e) => {
            return Err(GenInsertsFromFileError::ParserError(e));
        }
    }
}

#[derive(Debug)]
pub enum ForceWriteAllError {
    InvalidPath,
    WriteError,
    CreateFileError,
    CreateDirectoryError,
}

fn force_write_all(path: &str, data: &[u8]) -> Result<(), ForceWriteAllError> {
    let full_path = Path::new(path);

    let dir_path = full_path.parent();
    let dir_path = match dir_path {
        Some(p) => p,
        None => return Err(ForceWriteAllError::InvalidPath),
    };

    let create_dir = fs::create_dir_all(dir_path);
    if let Err(_) = create_dir {
        return Err(ForceWriteAllError::CreateDirectoryError);
    }

    let file = File::create(full_path);
    let mut file = match file {
        Ok(f) => f,
        Err(_) => return Err(ForceWriteAllError::CreateFileError),
    };

    let write_res = file.write_all(data);
    if let Err(_) = write_res {
        return Err(ForceWriteAllError::WriteError);
    }

    Ok(())
}
