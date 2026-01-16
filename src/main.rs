/*
add axum and other packages with features
cargo add axum tokio sqlx serde serde_json --features=tokio/macros,tokio/rt-multi-thread,tokio/net,sqlx/runtime-tokio-rustls,sqlx/postgres,sqlx/macros,serde/derive


*/

mod modules;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use bcrypt::DEFAULT_COST;
use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

use modules::users::{applications::dtos::CreateUserPayload, domain::entities::User};

use crate::modules::users::applications::dtos::{UpdateUserPayload, UserResponse};

#[tokio::main]
async fn main() {
    dotenv().ok(); // <-- PENTING

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
        .route(
            "/users/{id}",
            get(get_user).put(update_user).delete(delete_user),
        )
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

fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    // The hash function handles salting and cost settings internally.
    // DEFAULT_COST is 12, an integer between 4 and 31.
    bcrypt::hash(password, DEFAULT_COST)
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
) -> Result<axum::Json<Vec<UserResponse>>, (StatusCode, String)> {
    let rows = sqlx::query_as::<_, User>(
        r#"
        SELECT
            id,
            secret_id,
            username,
            email,
            password_hash,
            created_at,
            updated_at
        FROM users
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    // .fetch_all(&pool)
    // .await
    // .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()));

    let users = rows
        .into_iter()
        .map(|u| UserResponse {
            secret_id: u.secret_id,
            username: u.username,
            email: u.email,
            created_at: u.created_at,
            updated_at: u.updated_at,
        })
        .collect();

    Ok(Json(users))
}

// async fn list_users(
//     State(pool): State<PgPool>,
// ) -> Result<axum::Json<Vec<User>>, (StatusCode, String)> {
//     let users = sqlx::query_as::<_, User>("SELECT * FROM users")
//         .fetch_all(&pool)
//         .await
//         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

//     Ok(axum::Json(users))
// }

// CREATE USER
async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let hashed_password: String =
        hash_password(&payload.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(payload.username)
    .bind(payload.email)
    .bind(hashed_password)
    .fetch_one(&pool)
    .await
    .map(|u| (StatusCode::CREATED, Json(u)))
    .map_err(|_| StatusCode::NOT_FOUND)
}

// GET USER BY ID
async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

// UPDATE USER
async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<Json<User>, StatusCode> {
    sqlx::query_as::<_, User>(
        "UPDATE users SET username = $1, email = $2 WHERE id = $3 RETURNING *",
    )
    .bind(payload.username)
    .bind(payload.email)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map(Json)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// DELETE USER
async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
