/*
add axum and other packages with features
cargo add axum tokio sqlx serde serde_json --features=tokio/macros,tokio/rt-multi-thread,tokio/net,sqlx/runtime-tokio-rustls,sqlx/postgres,sqlx/macros,serde/derive


*/

use std::env;

use axum::{Router, http::StatusCode, routing::{get, post}};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let db_url:String = env::var("DATABASE_URL").expect("DATABASE_URL must be set"); 
    let pool = PgPoolOptions::new().connect(&db_url).await.expect("Failed to connect to Database");
    sqlx:: migrate!().run(&pool).await.expect("Migrations failed");

    // routingnya mirip dengan di express
    // let app = Router::new()
    // .route("/", get(root))
    // .route("/users", post(create_user).get(list_users))
    // .route("/users/{id}", get(get_user).put(update_user).delete(delete_user));

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    // println!("Listening on http://0.0.0.0:8000");
    // axum::serve(listener, app).await.unwrap();

}
