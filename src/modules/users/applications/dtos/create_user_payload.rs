use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub email: String,
    pub password: String,
}
