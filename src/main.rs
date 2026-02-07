use axum::{
    Router,
    routing::get,
    response::Html,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello", 
        get(|| async { Html("Hello <strong>World!!!</strong>") }),
    );

    // Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{addr}\n");
    
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    
    axum::serve(listener, routes_hello)
        .await
        .unwrap();
}