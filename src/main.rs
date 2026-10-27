mod handlers;

use crate::handlers::*;
use axum_extra::extract::cookie::Key;
use std::net::*;
use axum::{
    routing::{get, post},
    middleware,
    Router,
};
use sqlx::{ postgres::{PgPoolOptions, Postgres,}, pool::Pool };
use std::env;
use std::path::Path;
use axum_extra::extract::cookie::SignedCookieJar;
use axum::{
    extract::Request,
    response::{IntoResponse, Redirect, Response, Html},
};
use dotenv::dotenv;

#[derive(Clone)]
struct AppState {
    db: Pool<Postgres>,
}

impl AppState {
    fn from(pool: Pool<Postgres>) -> Self {
        Self { db: pool }
    }
}

#[tokio::main]
async fn main() {

    dotenv().ok();
    let db_url= env::var("DATABASE_URL").expect("DATABASE_URL not found");

    let pool = match PgPoolOptions::new()
        .max_connections(3)
        .connect(&db_url)
        .await {
            Ok(pool) => {
                println!("> created db connection pool.");
                pool
            },
            Err(e) => {
                eprintln!("Could not connect to database. -> {}", e);
                return
            },
        };
    
    let state = AppState::from(pool);

    let app = Router::new()
        .route("/", get(landing_handler))
        .with_state(state);

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid numer.");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            println!("> server running @ {}", {addr});
            listener
        },
        Err(e) => {
            eprintln!("> error binding TCP listener | {}", e);
            return;
        },
    };

    axum::serve(listener, app).await.expect("> could not serve app with listener.");
}


