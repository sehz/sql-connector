use anyhow::{Context, Result};
use async_trait::async_trait;
use url::Url;

use fluvio::Offset;
use fluvio_connector_common::{LocalBoxSink, Sink};
use fluvio_model_sql::Operation;

use crate::{config::DuckdbConfig, db::DuckDB};

#[derive(Debug)]
pub(crate) struct SqlSink {
    url: Url,
}

impl SqlSink {
    pub(crate) fn new(config: &DuckdbConfig) -> Result<Self> {
        let url = Url::parse(&config.url.resolve()?).context("unable to parse sql url")?;

        Ok(Self { url })
    }
}

#[async_trait]
impl Sink<Operation> for SqlSink {
    async fn connect(self, _offset: Option<Offset>) -> Result<LocalBoxSink<Operation>> {
        let db = DuckDB::connect(self.url.as_str()).await?;
        let unfold = futures::sink::unfold(db, |mut db: DuckDB, record: Operation| async move {
            db.execute(record).await?;
            Ok::<_, anyhow::Error>(db)
        });
        Ok(Box::pin(unfold))
    }
}
