pub use self::error::{Error, Result};
use axum::response::{Html, IntoResponse, Response};
use axum::{
    extract::{Path, Query},
    middleware,
    routing::get,
    Router,
};

use tower_cookies::CookieManagerLayer;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
mod error;
mod web;

#[derive(serde::Deserialize, Debug)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// for middleware
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        // Change {name} to :name
        .route("/hello2/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", ServeDir::new("./"))
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello <strong>{name}</strong>"))
}
