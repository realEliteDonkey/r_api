mod app_state;
mod handlers;
mod server_startup_utils;
mod shutdown;

use crate::app_state::AppState;
use crate::server_startup_utils::*;
use crate::shutdown::shutdown_sig;

use axum::{
    Router,
    extract::Request,
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
};
use axum_extra::extract::cookie::{Key, SignedCookieJar};
use dotenv::dotenv;
use sqlx::{
    pool::Pool,
    postgres::{PgPoolOptions, Postgres},
};
use std::{env, net::*, path::Path};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    dotenv().ok();

    let args: Vec<String> = std::env::args().collect();

    let host_ip = get_host_ip(args)
        .map_err(|status| std::io::Error::new(std::io::ErrorKind::Other, status.to_string()))?;

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let pool = gen_connection_pool(db_url)
        .await
        .map_err(|status| std::io::Error::new(std::io::ErrorKind::NotFound, status.to_string()))?;

    let state = AppState::from(pool);

    let app_task = tokio::task::spawn_blocking(move || generate_router(state));

    let port_task = tokio::task::spawn_blocking(move || get_env_port());

    let (app, port) = tokio::join!(app_task, port_task);

    let app = app?;
    let port = port?;
    let addr = SocketAddr::from((host_ip, port));

    let listener = gen_tcp_listener(addr)
        .await
        .map_err(|status| std::io::Error::new(std::io::ErrorKind::Other, status.to_string()))?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_sig())
        .await
        .expect("> could not serve app with listener.");

    Ok(())
}
