use wasmcomputev3_lib::http::{
    self,
    url::{filter::exact, Path},
};

pub struct GetAllBoard {}

impl GetAllBoard {
    pub fn new() -> Self {
        Self {}
    }

    fn handle(_: http::Request) -> http::Response {
        http::Response::new()
    }
}

impl wasmcomputev3_lib::http::Event for GetAllBoard {
    fn enabled(&self) -> bool {
        cfg!(feature = "get_all_board")
    }

    fn url(&self) -> String {
        Path::get()
            .pipe(exact("api"))
            .pipe(exact("board"))
            .into_route()
            .endpoint()
            .url()
    }

    fn handle(
        &self,
        request: wasmcomputev3_lib::http::Request,
    ) -> wasmcomputev3_lib::http::Response {
        Path::get()
            .pipe(exact("api"))
            .pipe(exact("board"))
            .into_route()
            .endpoint()
            .handle(request, Self::handle)
    }
}

pub struct CreateBoard {}

impl CreateBoard {
    pub fn new() -> Self {
        Self {}
    }

    fn serve(req: http::Request) -> http::Response {
        http::Response::new()
    }
}

impl wasmcomputev3_lib::http::Event for CreateBoard {
    fn enabled(&self) -> bool {
        cfg!(feature = "create_board")
    }

    fn url(&self) -> String {
        Path::post()
            .pipe(exact("api"))
            .pipe(exact("board"))
            .into_route()
            .endpoint()
            .url()
    }

    fn handle(
        &self,
        request: wasmcomputev3_lib::http::Request,
    ) -> wasmcomputev3_lib::http::Response {
        Path::post()
            .pipe(exact("api"))
            .pipe(exact("board"))
            .into_route()
            .endpoint()
            .handle(request, Self::serve)
    }
}
