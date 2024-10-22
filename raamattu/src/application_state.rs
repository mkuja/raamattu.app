use std::sync::Arc;
use sqlx::{PgPool};
use tantivy::{Index, IndexReader};


#[derive(Clone)]
pub struct ApplicationState {
    pub pg_client: raamattu_db::client::Client,
    // pub index: Arc<Index>,
    // pub reader: Arc<IndexReader>,
}


