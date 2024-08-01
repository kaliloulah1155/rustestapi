use axum::{Router, routing::get};

pub fn create_routes() -> Router {
    Router::new().route("/hello", get(|| async { "Hello world 1 !!" }))
}
