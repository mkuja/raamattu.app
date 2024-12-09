use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Clone, Debug)]
pub struct BackendState {
    pub database_connection: PgPool,
}

impl BackendState {
    pub async fn new(db_connection_str: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&db_connection_str)
            .await
            .expect("can't connect to database");
        Self {
            database_connection: pool,
        }
    }
}
