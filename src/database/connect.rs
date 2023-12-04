use eyre::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type DB = Pool<Postgres>;

pub async fn connect_to_database(database_uri: &str) -> Result<DB> {
    Ok(PgPoolOptions::new()
        .max_connections(5)
        .connect(database_uri)
        .await?)
}
