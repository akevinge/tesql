use serde::{de::IntoDeserializer, Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
#[serde(remote = "DataSqlKey")]
pub enum DataSqlKey {
    PKey,
    FKey,
}

impl<'de> Deserialize<'de> for DataSqlKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "pKey" => Ok(DataSqlKey::PKey),
            "fKey" => Ok(DataSqlKey::FKey),
            _ => DataSqlKey::deserialize(s.into_deserializer()),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(remote = "DataSqlType")]
pub enum DataSqlType {
    UUID,
    Text,
    Array,
}

impl<'de> Deserialize<'de> for DataSqlType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "uuid" => {
                return Ok(DataSqlType::UUID);
            }
            "text" => Ok(DataSqlType::Text),
            "array" => Ok(DataSqlType::Text),
            _ => DataSqlType::deserialize(s.into_deserializer()),
        }
    }
}
