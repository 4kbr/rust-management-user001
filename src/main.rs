/*
add axum and other packages with features
cargo add axum tokio sqlx serde serde_json --features=tokio/macros,tokio/rt-multi-thread,tokio/net,sqlx/runtime-tokio-rustls,sqlx/postgres,sqlx/macros,serde/derive


*/

use axum::{http::StatusCode};
fn main() {
    println!("Hello, world!");
}
