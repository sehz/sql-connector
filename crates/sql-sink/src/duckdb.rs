use anyhow::Result;
use duckdb::{params_from_iter, Connection};

use fluvio_model_sql::Value;

pub(crate) fn insert(conn: &Connection, table: &str, values: Vec<Value>) -> Result<()> {
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
