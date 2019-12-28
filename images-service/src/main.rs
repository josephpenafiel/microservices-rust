#[macro_use]
extern crate lazy_static;
mod handler;
use hyper::{service::service_fn, Server};
use std::{fs, path::Path};
use futures::{Future};
use handler::microservice_handler;
fn main() {
    let files = Path::new("./files");
    fs::create_dir(files).ok();
    let addr = ([127, 0, 0, 1], 8080).into();
    let server =
        Server::bind(&addr)
        .serve(move || {
            service_fn(move |req| microservice_handler(req, &files))
        });
    let server = server.map_err(drop);
    hyper::rt::run(server);
}