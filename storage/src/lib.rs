pub use sea_orm;

use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

mod mutation;
mod query;

pub use mutation::*;
pub use query::*;
