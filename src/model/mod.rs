use std::sync::Arc;

use conventually::{Database, Tree, U64};

use self::{board::Board, column::Column, ticket::Ticket};

mod board;
mod column;
mod ticket;

pub struct State {
    boards: Tree<U64, Board>,
    columns: Tree<U64, Column>,
    tickets: Tree<U64, Ticket>,
}

impl State {
    pub async fn restore() -> anyhow::Result<Arc<State>> {
        let mut database = Database::new();

        let boards = board::into_tree(&mut database).await?;
        let columns = column::into_tree(&mut database).await?;
        let tickets = ticket::into_tree(&mut database).await?;

        Ok(Arc::new(State {
            boards,
            columns,
            tickets,
        }))
    }
}
