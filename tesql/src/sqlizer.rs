use linked_hash_map::LinkedHashMap;
use serde_json::Value;

#[derive(Debug)]
pub enum SqlType {
    UUID,
    Text,
    Array,
}

#[derive(Debug)]
pub struct SqlColumn {
    pub name: String,
    pub r#type: SqlType,
}

#[derive(Debug)]
pub struct SqlInsertModel {
    pub table_name: String,
    pub columns: Vec<SqlColumn>,
    pub inserts: Vec<LinkedHashMap<String, Value>>,
}

#[derive(Debug)]
pub enum StringifyInsertError {
    ValueNotFoundForColumn(String),
    Invalid,
}

pub fn sqlize_insert(m: &SqlInsertModel) -> Result<String, StringifyInsertError> {
    let mut sql = format!(
        "INSERT INTO \"{}\" ({})\nVALUES ",
        m.table_name,
        m.columns
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (i, x)| {
                if i == 0 {
                    x.name.clone()
                } else {
                    format!("{}, {}", acc, x.name)
                }
            })
    )
    .to_string();

    for (i, rows) in m.inserts.iter().enumerate() {
        let mut values = String::from("(");
        for (i, c) in m.columns.iter().enumerate() {
            let column_name = &c.name;
            let row_val = rows.get(column_name);

            match row_val {
                Some(val) => match val {
                    Value::Bool(val) => {
                        values.push_str(&val.to_string());
                    }
                    Value::Array(val) => values.push_str(&format!("'{}'", &stringify_vec(val))),
                    Value::String(val) => {
                        values.push_str(&format!("'{}'", val));
                    }
                    Value::Null => {}
                    _ => return Err(StringifyInsertError::Invalid),
                },
                None => {
                    return Err(StringifyInsertError::ValueNotFoundForColumn(
                        column_name.clone(),
                    ))
                }
            };

            if i != m.columns.len() - 1 {
                values.push_str(", ");
            }
        }
        values.push_str(if i == m.inserts.len() - 1 {
            ");"
        } else {
            "), "
        });
        sql.push_str(&values);
    }

    Ok(sql)
}

pub fn compose_inserts(sql_models: Vec<SqlInsertModel>) -> Result<String, StringifyInsertError> {
    let mut sql_str = String::new();
    let is_multiline = sql_models.len() > 1;

    for sm in sql_models.iter() {
        match sqlize_insert(sm) {
            Ok(sql) => {
                let mut insert_str = if is_multiline {
                    format!("{}\n\n", sql.to_owned())
                } else {
                    format!("{}", sql.to_owned())
                };
                sql_str.push_str(insert_str.as_mut_str());
            }
            Err(e) => return Err(e),
        }
    }
    Ok(sql_str)
}

fn stringify_vec(v: &Vec<Value>) -> String {
    let mut str = String::from("{");
    for (i, x) in v.iter().enumerate() {
        let is_last = i == v.len() - 1;
        match x {
            Value::Array(ar) => {
                let s = if is_last {
                    stringify_vec(ar)
                } else {
                    format!("{}, ", stringify_vec(ar))
                };
                str.push_str(&s);
            }
            _ => {
                let s = if is_last {
                    format!("{}", &x.to_string())
                } else {
                    format!("{}, ", &x.to_string())
                };
                str.push_str(&s.to_string());
            }
        }
    }
    str.push_str("}");
    str
}

#[cfg(test)]
mod sqlizer_tests {
    use serde_json::{json, Value};

    use crate::sqlizer::{self};

    #[test]
    fn test_stringify_vec() {
        let vec: Vec<Value> = vec![json!(1), json!(2), json!(3)];
        let str = sqlizer::stringify_vec(&vec);
        assert_eq!(String::from("{1, 2, 3}"), str);

        // nested arrays
        let vec: Vec<Value> = vec![
            json!(vec![String::from("hello")]),
            json!(vec![String::from("world")]),
        ];
        let str = sqlizer::stringify_vec(&vec);
        assert_eq!(String::from("{{\"hello\"}, {\"world\"}}"), str);
    }
}
