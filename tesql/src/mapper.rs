use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;
use serde_json::Value;

use crate::{
    data::{
        parser::{DataInsertModel, RawDataModel},
        types::DataSqlType,
    },
    json::{JsonDataArray, JsonRawModel},
    sqlizer::{SqlColumn, SqlInsertModel, SqlType},
};

pub fn map_data_to_inserts(model: &RawDataModel) -> Vec<SqlInsertModel> {
    let mut sql_models: Vec<SqlInsertModel> = vec![];

    for (model_name, data_insert_model) in model.data.iter() {
        sql_models.push(SqlInsertModel {
            table_name: model_name.clone(),
            columns: data_insert_model
                .clone()
                .columns
                .into_iter()
                .map(|x| SqlColumn {
                    name: x.name.clone(),
                    r#type: match x.r#type {
                        DataSqlType::Text => SqlType::Text,
                        DataSqlType::UUID => SqlType::UUID,
                        DataSqlType::Array => SqlType::Array,
                    },
                })
                .collect::<Vec<SqlColumn>>(),
            inserts: data_insert_model.inserts.clone(),
        })
    }

    return sql_models;
}

pub fn map_insert_to_json(
    key: &String,
    insert_model: &DataInsertModel,
) -> HashMap<String, JsonDataArray> {
    let mut inserts: JsonDataArray = vec![];

    for map in insert_model.inserts.iter() {
        inserts.push(map.clone());
    }

    HashMap::from([(key.clone(), inserts)])
}

pub fn map_data_to_json(data_model: &RawDataModel) -> JsonRawModel {
    let mut data = LinkedHashMap::<String, JsonDataArray>::new();

    for (k, v) in data_model.data.iter() {
        let mut inserts: Vec<LinkedHashMap<String, Value>> = vec![];

        for i in &v.inserts {
            inserts.push(i.clone());
        }

        data.insert(k.clone(), inserts);
    }

    JsonRawModel { data: data }
}
