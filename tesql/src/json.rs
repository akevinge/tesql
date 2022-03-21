use linked_hash_map::LinkedHashMap;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct JsonRawModel {
    pub data: LinkedHashMap<String, JsonDataArray>,
}

pub type JsonDataArray = Vec<LinkedHashMap<String, Value>>;
