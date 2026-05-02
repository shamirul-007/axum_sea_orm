use axum::{ Json, extract::{ Path, State } };
use sea_orm::JsonValue;

use crate::{ modules::post::service::PostService, state::AppState };

pub async fn get_posts(State(state): State<AppState>) -> axum::Json<JsonValue> {
    let post = PostService::new(state.db).get_posts().await;

    match post {
        Ok(value) =>
            Json(
                serde_json::json!({
                "status":"success",
                "data": value
            })
            ),
        Err(_) =>
            Json(
                serde_json::json!({
            "status": "failed",
            "data": null
        })
            ),
    }
}

pub async fn get_post(State(state): State<AppState>, Path(id): Path<i32>) -> axum::Json<JsonValue> {
    let post = PostService::new(state.db).get_post(id).await;

    match post {
        Ok(Some(value)) => {
            Json(
                serde_json::json!({
                "status":"success",
                "data":value,
                "message":"data fetched success"
            })
            )
        }
        Ok(None) => {
            Json(
                serde_json::json!({
                "status":"failed",
                "data":null,
                "message":"no post available"
            })
            )
        }
        Err(_) => {
            Json(
                serde_json::json!({
                "status":"failed",
                "data":null,
                "message":"internal server error"
            })
            )
        }
    }
}
