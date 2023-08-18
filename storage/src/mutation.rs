use ::models::bounties;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, Set};

pub struct Mutation;

impl Mutation {
    pub async fn add_bounty(
        db: &DbConn,
        new_bounty: bounties::Model,
    ) -> Result<bounties::Model, DbErr> {
        bounties::ActiveModel {
            id: Set(new_bounty.id.to_owned()),
            description: Set(new_bounty.description.to_owned()),
            price: Set(new_bounty.price.to_owned()),
        }
        .insert(db)
        .await
    }
}
