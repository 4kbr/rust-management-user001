/*
add axum and other packages with features
cargo add axum tokio sqlx serde serde_json --features=tokio/macros,tokio/rt-multi-thread,tokio/net,sqlx/runtime-tokio-rustls,sqlx/postgres,sqlx/macros,serde/derive


*/

mod modules;
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

use modules::users::{applications::dtos::CreateUserPayload, domain::entities::User};

#[tokio::main]
async fn main() {
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to Database");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migrations failed");

    // routingnya mirip dengan di express
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user).get(list_users))
        // .route(
        //     "/users/{id}",
        //     get(get_user).put(update_user).delete(delete_user),
        // );
        .with_state(pool);
    /*
    kenapa bind ke 0.0.0.0 ? biar bisa diakses dari luar container (jika dijalankan di dalam container)
    8000 itu portnya, bisa diganti sesuai kebutuhan
    */
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Listening on http://0.0.0.0:8000");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

/*
kenapa &'static str? bukan &str atau String?
karena string literal di Rust itu punya lifetime 'static secara default, yang berarti mereka hidup selama program berjalan.
Jadi ketika kita mengembalikan &'static str, kita memastikan bahwa referensi tersebut valid selama seluruh runtime program.
btw "&'static str" itu disebut string slice dengan lifetime 'static.
*/
async fn root() -> &'static str {
    "Welcome to user management API!"
}
// GET ALL USERS
async fn list_users(
    State(pool): State<PgPool>,
) -> Result<axum::Json<Vec<User>>, (StatusCode, String)> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .map(axum::Json)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

// CREATE USER
async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2) RETURNING *",
    )
    .bind(payload.username)
    .bind(payload.email)
    .bind(payload.password)
    .fetch_one(&pool)
    .await
    .map(|u| (StatusCode::CREATED, Json(u)))
    .map_err(|_| StatusCode::NOT_FOUND)
}
