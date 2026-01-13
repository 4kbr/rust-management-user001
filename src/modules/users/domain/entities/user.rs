use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub secret_id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}
