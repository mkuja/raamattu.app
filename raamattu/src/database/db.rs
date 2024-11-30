use sqlx::PgPool;

#[derive(Clone)]
pub struct Client {
    pub connection_pool: PgPool,
}
