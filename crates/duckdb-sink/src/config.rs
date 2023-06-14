use fluvio_connector_common::{connector, secret::SecretString};

#[derive(Debug)]
#[connector(config, name = "duckdb")]
pub(crate) struct DuckdbConfig {
    pub url: SecretString,
}
