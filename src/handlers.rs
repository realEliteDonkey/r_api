use axum::body::Body;
use axum::http::{Request, Response};
use axum::response::{Html, IntoResponse};
use std::pin::Pin;

type Handler = fn(axum::http::Request<Body>) -> Pin<Box<dyn Future<Output = Response<()>> + Send>>;

pub async fn post_login_handler() -> impl IntoResponse {
    //split into separate private post handler
    "(GET) Login Handler Called"
}

pub async fn get_landing_handler() -> impl IntoResponse {
    //split into separate private get/post handlers
    // "(GET) Landing Handler Called"
    Html(include_str!("../frontend/index.html"))
}

pub async fn post_landing_handler() -> impl IntoResponse {
    //split into separate private get/post handlers
    "(POST) Landing Handler Called"
}

pub async fn get_create_user_handler() -> impl IntoResponse {
    //split into separate private get/post handlers
    "(GET) Create User Handler Called"
}

pub async fn post_create_user_handler() -> impl IntoResponse {
    //split into separate private get/post handlers
    "(POST) Create User Handler Called"
}
