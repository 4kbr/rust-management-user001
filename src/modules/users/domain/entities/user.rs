use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct User {
    id: i64,
    secret_id: String,
    username: String,
    email: String,
    password: String,
    // created_at:
    // updated_at:
}
