use serde::Deserialize;
#[derive(Deserialize)]
struct CreateUserPayload {
    username: String,
    email: String,
    password: String,
}
