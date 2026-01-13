use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUserPayload {
    pub username: String,
    pub email: String,
}
