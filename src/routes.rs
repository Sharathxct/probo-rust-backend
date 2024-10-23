use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use axum::{
    routing::{get, post},
    Router,
};
use serde_json::json;

pub fn create_routes() -> Router<()> {
    Router::new()
        .route("/:user_id", get(hello_world))
        .route("/user/create/:userId", post(create_userx))
}

pub async fn hello_world(Path(user_id): Path<String>) -> impl IntoResponse {
    println!("req received {}", user_id);
    let response = json!({
        "msg": "received",
        "user_id": user_id,
    });

    (StatusCode::OK, Json(response))
}

pub async fn create_userx(Path(user_id): Path<String>) -> impl IntoResponse {
    // UserData::new(user_id.clone());
    (
        StatusCode::CREATED,
        Json(json!({ "msg": "User {} created" })),
    )
}
