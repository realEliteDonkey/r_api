use sqlx::{pool::Pool, postgres::Postgres};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

impl AppState {
    pub fn from(pool: Pool<Postgres>) -> Self {
        Self { db: pool }
    }
}
