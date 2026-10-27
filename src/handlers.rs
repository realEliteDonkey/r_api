use axum::response::IntoResponse;

pub async fn landing_handler() -> impl IntoResponse {
    "Landing Handler Called"
}

pub async fn login_handler() -> impl IntoResponse {
    "Login Handler Called"
}

pub async fn create_user_handler() -> impl IntoResponse {
    "Create User Handler Called"
}
