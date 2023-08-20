use crate::models::{
    bounties::ActiveModel as BountiesActiveModel, bounties::Model as BountiesModel,
    prelude::Bounties as BountiesEntity,
};
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct BountiesRequest {
    description: String,
    price: f64,
}

#[derive(Serialize)]
pub struct BountiesResponse {
    id: i32,
    description: String,
    price: f64,
}

impl From<BountiesModel> for BountiesResponse {
    fn from(bounty: BountiesModel) -> Self {
        BountiesResponse {
            id: bounty.id,
            description: bounty.description,
            price: bounty.price,
        }
    }
}

pub async fn get_all_bounties(
    State(database): State<DatabaseConnection>,
) -> Result<Json<Vec<BountiesResponse>>, StatusCode> {
    let bounties = BountiesEntity::find()
        .all(&database)
        .await
        .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|bounty| BountiesResponse::from(bounty))
        .collect();

    Ok(Json(bounties))
}

pub async fn add_bounty(
    State(database): State<DatabaseConnection>,
    Json(bounty): Json<BountiesRequest>,
) -> StatusCode {
    let new_bounty = BountiesActiveModel {
        description: Set(bounty.description),
        price: Set(bounty.price),
        ..Default::default()
    };

    let res = new_bounty.save(&database).await;

    match res {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
