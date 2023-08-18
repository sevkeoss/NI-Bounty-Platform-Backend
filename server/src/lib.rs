use std::{env, net::SocketAddr};

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

use handlers::bounties::{add_bounty, get_bounties_handler};
use storage::{sea_orm::Database, AppState};

use dotenv::dotenv;

#[tokio::main]
async fn start() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    println!("db_url: {}", db_url);

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let state = AppState { conn };

    let app = Router::new()
        .route("/bounties", get(get_bounties_handler))
        .route("/bounty", post(add_bounty))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    tracing::debug!("listening on {}", addr);

    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn main() {
    start();
}
