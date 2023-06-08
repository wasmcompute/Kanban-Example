use std::path::PathBuf;

use wasmcomputev3_lib::http::{
    self,
    url::{
        filter::{exact, static_mount},
        Path,
    },
};

pub struct StaticFiles {
    path: PathBuf,
}

impl StaticFiles {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        Self { path }
    }

    fn serve(_: http::Request, file: Vec<u8>) -> http::Response {
        http::Response::new()
            .header("content-type", "text/html")
            .body(file)
    }
}

impl wasmcomputev3_lib::http::Event for StaticFiles {
    fn enabled(&self) -> bool {
        cfg!(feature = "static_files")
    }

    fn url(&self) -> String {
        Path::get()
            .pipe(exact("static"))
            .pipe(static_mount("file", self.path.as_path().to_str().unwrap()))
            .into_route()
            .endpoint()
            .url()
    }

    fn handle(
        &self,
        request: wasmcomputev3_lib::http::Request,
    ) -> wasmcomputev3_lib::http::Response {
        Path::get()
            .pipe(exact("static"))
            .pipe(static_mount("file", self.path.as_path().to_str().unwrap()))
            .into_route()
            .resolve::<Vec<u8>, _, _>(|f| &f.0)
            .endpoint()
            .handle(request, Self::serve)
    }
}
