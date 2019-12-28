mod rest;
mod rest_regex;
pub use rest::microservice_handler;
pub use rest_regex::microservice_handler_regex;

use hyper::{Response, Body, Error, StatusCode,};
use slab::Slab;
use std::sync::{Arc, Mutex};
use std::fmt;

pub type UserId = u64;
pub type UserDb = Arc<Mutex<Slab<UserData>>>;
pub struct UserData;

impl fmt::Display for UserData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("{}")
    }
}

pub fn response_with_code(status_code: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(Body::empty())
        .unwrap()
}