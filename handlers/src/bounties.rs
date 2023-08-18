use axum::{extract::State, http::StatusCode, Json};
use models::bounties::{CreateBountyModel, Model};
use storage::AppState;
use storage::Mutation as MutationCore;
use utils::ids::generate_uuid;

use storage::Query as QueryCore;

pub async fn get_bounties_handler(state: State<AppState>) -> (StatusCode, Json<Vec<Model>>) {
    let bounties = QueryCore::get_bounties(&state.conn)
        .await
        .expect("Cannot find any bounties");

    (StatusCode::OK, Json(bounties))
}

pub async fn add_bounty(
    state: State<AppState>,
    Json(payload): Json<CreateBountyModel>,
) -> (StatusCode, Json<Model>) {
    let input = payload;
    let bounty = Model {
        id: generate_uuid(),
        description: input.description,
        price: input.price,
    };

    let bounty = MutationCore::add_bounty(&state.conn, bounty)
        .await
        .expect("Cannot add new bounty");

    (StatusCode::OK, Json(bounty))
}
