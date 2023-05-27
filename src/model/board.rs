use conventually::{Database, Tree, U64};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Board {
    name: String,
}

impl Board {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub async fn into_tree(db: &mut Database) -> anyhow::Result<Tree<U64, Board>> {
    db.create("Boards")?.unwrap().await
}
