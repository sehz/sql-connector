use serde::Deserialize;
use serde::Serialize;

/// Top-level list of supported operations in the SQL model.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Operation {
    Insert { table: String, values: Vec<Value> },
}

/// Value with SQL column name and supported SQL type.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Value {
    pub column: String,
    pub raw_value: String,
    #[serde(rename = "type")]
    pub type_: Type,
}


/// Supported SQL data types.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Type {
    Bool,
    Char,

    SmallInt,
    Int,
    BigInt,

    Float,
    DoublePrecision,

    Text,
    Bytes,

    Numeric,

    Timestamp,
    Date,
    Time,

    Uuid,

    Json,
}

mod duckdb {

    use duckdb::{ToSql, types::ToSqlOutput};
    use duckdb::types::{Value as DuckValue, TimeUnit};

    use super::Value;

    impl ToSql for Value {
        fn to_sql(&self) -> duckdb::Result<duckdb::types::ToSqlOutput<'_>> {
            
            if self.raw_value == "NULL" {
                return Ok(ToSqlOutput::from(duckdb::types::Null));
            }
            match self.type_ {
                crate::Type::Bool => {
                    let value: bool = self.raw_value.parse::<bool>().map_err(|err| duckdb::Error::ToSqlConversionFailure(Box::new(err)))?;
                    Ok(ToSqlOutput::from(value))
                }
                crate::Type::Char => {
                    Ok(ToSqlOutput::from(self.raw_value.as_str()))
                }
                crate::Type::SmallInt => {
                    let value: i16 = self.raw_value.parse::<i16>().map_err(|err| duckdb::Error::ToSqlConversionFailure(Box::new(err)))?;
                    Ok(ToSqlOutput::from(value))
                }
                crate::Type::Int => {
                    let value: i32 = self.raw_value.parse::<i32>().map_err(|err| duckdb::Error::ToSqlConversionFailure(Box::new(err)))?;
                    Ok(ToSqlOutput::from(value))
                }
                crate::Type::BigInt => {
                    let value: i64 = self.raw_value.parse::<i64>().map_err(|err| duckdb::Error::ToSqlConversionFailure(Box::new(err)))?;
                    Ok(ToSqlOutput::from(value))
                }
                crate::Type::Float => {
                    let value: f32 = self.raw_value.parse::<f32>().map_err(|err| duckdb::Error::ToSqlConversionFailure(Box::new(err)))?;
                    Ok(ToSqlOutput::from(value))
                }
                crate::Type::DoublePrecision => {
                    let value: f64 = self.raw_value.parse::<f64>().map_err(|err| duckdb::Error::ToSqlConversionFailure(Box::new(err)))?;
                    Ok(ToSqlOutput::from(value))
                }
                crate::Type::Text => {
                    Ok(ToSqlOutput::from(self.raw_value.as_str()))
                }
                crate::Type::Bytes => {
                    Ok(ToSqlOutput::from(self.raw_value.as_bytes()))
                }
                crate::Type::Numeric => todo!(),
                crate::Type::Timestamp =>  {
                    // 2023-03-03T18:30:18.679Z
                  //  println!("parsing timestamp: {}",self.raw_value.as_str());
                    let timestamp = chrono::DateTime::parse_from_rfc3339(self.raw_value.as_str()).map_err(|err| duckdb::Error::ToSqlConversionFailure(Box::new(err)))?;
                  //  println!("timestamp: {:#?}",timestamp);
                    Ok(ToSqlOutput::Owned(DuckValue::Timestamp(TimeUnit::Millisecond, timestamp.timestamp_millis())))
                }
                crate::Type::Date => todo!(),
                crate::Type::Time => todo!(),
                crate::Type::Uuid => todo!(),
                crate::Type::Json => todo!(),
            }
        }
    }


}