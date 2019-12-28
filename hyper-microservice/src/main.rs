#![allow(unused_imports)]
use futures::{future, Future};
use hyper::{service::service_fn, Body, Error, Method, Request, Response, Server, StatusCode};
use slab::Slab;
use std::fmt;
use std::sync::{Arc, Mutex};

mod router;
mod index;

// use router::microservice_handler;
use router::microservice_handler_regex;

//cargo watch -x "run"
fn main() {
    let user_db = Arc::new(Mutex::new(Slab::new()));
    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(move || {
        let user_db = user_db.clone();
        service_fn(move |req| microservice_handler_regex(req, &user_db))
    });
    let server = server.map_err(drop);
    hyper::rt::run(server);
}
