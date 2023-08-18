use ::models::{bounties, bounties::Entity as Bounties};
use sea_orm::{DbConn, DbErr, EntityTrait};

pub struct Query;

impl Query {
    pub async fn get_bounties(db: &DbConn) -> Result<Vec<bounties::Model>, DbErr> {
        Bounties::find().all(db).await
    }
}
