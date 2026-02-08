use crate::{Error, Result, web};
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

/// Returns the login routes for the API
pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, Json(payload): Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Implement real db/auth logic.
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // TODO: Set cookies / generate JWT
     cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

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
