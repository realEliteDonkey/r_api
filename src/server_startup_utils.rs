use crate::app_state::AppState;
use crate::handlers::*;

use axum::{Router, http::StatusCode, routing::get};
use sqlx::{
    pool::Pool,
    postgres::{PgPoolOptions, Postgres},
};
use std::env;
use std::net::*;
use tower_http::services::ServeDir;

pub async fn gen_connection_pool(db_url: String) -> Result<Pool<Postgres>, StatusCode> {
    return match PgPoolOptions::new()
        .max_connections(3)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            println!("> created db connection pool.");
            Ok(pool)
        }
        Err(e) => {
            eprintln!("Could not connect to database. -> {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };
}

pub fn generate_router(state: AppState) -> Router {
    println!("> Generating router...");
    let router = Router::new()
        .route("/", get(get_landing_handler))
        .nest_service("/frontend", ServeDir::new("frontend"))
        .with_state(state);
    println!("> Router created.");
    router
}

pub fn get_env_port() -> u16 {
    println!("> Getting PORT...");
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or_else(|e| {
            eprintln!("> Error parsing port u16 variable | {e}");
            8080 as u16
        });
    println!("> Fetched PORT");
    port
}

pub fn print_args(args: &Vec<String>) {
    for arg in args {
        print!("{arg} ");
    }
    println!();
}

pub fn get_host_ip(args: Vec<String>) -> Result<[u8; 4], StatusCode> {
    const BETA_IP: [u8; 4] = [127, 0, 0, 1];
    const PROD_IP: [u8; 4] = [0, 0, 0, 0];

    println!("> Fetching host_ip from args.");
    if args.len() > 2 {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    print_args(&args);
    if args.len() == 1 || args[1].trim().to_ascii_lowercase().eq("betaip") {
        println!("> Fetched host_ip.");
        return Ok(BETA_IP);
    } else if args[1].trim().to_ascii_lowercase().eq("prodip") {
        println!("> Fetched host_ip.");
        return Ok(PROD_IP);
    }
    return Err(StatusCode::INTERNAL_SERVER_ERROR);
}

pub async fn gen_tcp_listener(addr: SocketAddr) -> Result<tokio::net::TcpListener, StatusCode> {
    match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            println!("> server running @ {:?}", addr);
            Ok(listener)
        }
        Err(_) => {
            eprintln!("> error binding TCP listener");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}
