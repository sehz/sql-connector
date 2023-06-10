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


    use super::Value;

    impl ToSql for Value {
        fn to_sql(&self) -> duckdb::Result<duckdb::types::ToSqlOutput<'_>> {
            match self.type_ {
                crate::Type::Bool => todo!(),
                crate::Type::Char => todo!(),
                crate::Type::SmallInt => todo!(),
                crate::Type::Int => {
                    let value: i32 = self.raw_value.parse::<i32>().map_err(|err| duckdb::Error::ToSqlConversionFailure(Box::new(err)))?;
                    Ok(ToSqlOutput::from(value))
                }
                crate::Type::BigInt => todo!(),
                crate::Type::Float => {
                    let value: f32 = self.raw_value.parse::<f32>().map_err(|err| duckdb::Error::ToSqlConversionFailure(Box::new(err)))?;
                    Ok(ToSqlOutput::from(value))
                }
                crate::Type::DoublePrecision => todo!(),
                crate::Type::Text => todo!(),
                crate::Type::Bytes => todo!(),
                crate::Type::Numeric => todo!(),
                crate::Type::Timestamp => todo!(),
                crate::Type::Date => todo!(),
                crate::Type::Time => todo!(),
                crate::Type::Uuid => todo!(),
                crate::Type::Json => todo!(),
            }
        }
    }


}