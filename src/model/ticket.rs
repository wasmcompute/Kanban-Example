use std::time::SystemTime;

use conventually::{Database, Tree, ID, U64};

use super::column::Column;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Ticket {
    column: ID<U64, Column>,
    name: String,

    deleted: bool,

    created_at: u128,
    updated_at: u128,
}

impl Ticket {
    pub fn new(column: ID<U64, Column>, name: impl ToString) -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        Self {
            column,
            name: name.to_string(),
            deleted: false,
            created_at: now,
            updated_at: now,
        }
    }
}

pub async fn into_tree(db: &mut Database) -> anyhow::Result<Tree<U64, Ticket>> {
    db.create("Tickets")?.unwrap().await
}
