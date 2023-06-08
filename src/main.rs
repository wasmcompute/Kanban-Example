use api::{CreateBoard, GetAllBoard, StaticFiles};
use model::State;

mod api;
mod model;

fn main() {}

async fn run() {
    let state = State::restore().await.unwrap();

    let static_files = StaticFiles::new("./static");
    let get_all_boards = GetAllBoard::new();
    let create_board = CreateBoard::new();

    let app = wasmcomputev3_lib::App::new()
        .add_endpoint(static_files)
        .add_endpoint(get_all_boards)
        .add_endpoint(create_board);
    wasmcomputev3_lib::register(app);
}
