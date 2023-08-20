use std::net::SocketAddr;

mod models;
mod routes;

use dotenvy::dotenv;

use crate::routes::create_routes;

pub async fn start() {
    dotenv().ok();

    let app = match create_routes().await {
        Ok(router) => router,
        Err(err) => panic!("Could not connect to db: {}", err),
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
