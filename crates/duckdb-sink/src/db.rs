

use anyhow::{Result};

use fluvio_connector_common::tracing::{info, error};
use fluvio_model_sql::{Operation, Value};

use duckdb::{Connection as DuckDbConnection, params_from_iter};


pub struct DuckDB(DuckDbConnection);

impl DuckDB {
    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        info!(url, "opening duckdb");

        let conn = DuckDbConnection::open(url)?;
        Ok(Self(conn))
    }

    pub async fn execute(&mut self, operation: Operation) -> anyhow::Result<()> {
        match operation {
            Operation::Insert { table, values } => {
                self.insert(table, values).await?;
            }
        }
        Ok(())
    }

    async fn insert(&mut self, table: String, values: Vec<Value>) -> anyhow::Result<()> {
        
        if let Err(err) = insert(&self.0, &table, values) {
            error!("unable to insert duckdb: {}", err);
        }
        Ok(())
    }
}



pub(crate) fn insert(conn: &DuckDbConnection, table: &str, values: Vec<Value>) -> Result<()> {
    let mut query = String::from("INSERT INTO ");
    query.push_str(table);
    query.push_str(" (");
    for value in &values {
        query.push_str(&value.column);
        query.push_str(",");
    }
    query.pop();
    query.push_str(") ");
    query.push_str(" VALUES (");
    for _ in 0..values.len() {
        query.push_str("?,");
    }
    query.pop();
    query.push_str(")");

    let mut stmt = conn.prepare(&query)?;
    let params = params_from_iter(&values);
    stmt.execute(params)?;

    Ok(())
}
