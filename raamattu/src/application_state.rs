use crate::database;

#[derive(Clone)]
pub struct ApplicationState {
    pub pg_client: database::db::Client,
}


