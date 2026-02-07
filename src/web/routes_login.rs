use crate::{Error, Result};
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

/// Returns the login routes for the API
pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(Json(payload): Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Implement real db/auth logic.
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // TODO: Set cookies / generate JWT

    // Create the success body
    let body = json!({
        "result": {
            "success": true
        }
    });

    Ok(Json(body))
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub pwd: String,
}
