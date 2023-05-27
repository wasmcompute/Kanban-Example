use conventually::{Database, Tree, U64};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Column {
    name: String,
    position: usize,
}

impl Column {
    fn new(name: String, position: usize) -> Self {
        Self { name, position }
    }
}

impl std::fmt::Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub async fn into_tree(db: &mut Database) -> anyhow::Result<Tree<U64, Column>> {
    db.create("Columns")?.unwrap().await
}
