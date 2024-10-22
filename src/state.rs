use std::sync::Mutex;
use sqlx::PgPool;
use crate::models::tracker::Tracker;

pub struct AppState {
    pub tracker: Mutex<Tracker>,
    pub db_pool: PgPool,
}