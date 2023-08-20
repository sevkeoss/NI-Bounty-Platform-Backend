mod bounties;

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use bounties::{add_bounty, get_all_bounties};

use sea_orm::{Database, DbErr};
use tower_http::cors::{Any, CorsLayer};

pub async fn create_routes() -> Result<Router, DbErr> {
    let db_url = dotenvy::var("DATABASE_URL").unwrap();

    let db = Database::connect(db_url).await?;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    Ok(Router::new()
        .route("/bounty", get(get_all_bounties))
        .route("/bounty", post(add_bounty))
        .layer(cors)
        .with_state(db))
}
