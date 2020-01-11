#![allow(dead_code)]
use slab::Slab;
use futures::{Future, future,};
use hyper::{Request, Response, Body, Error, Method, StatusCode,};
use super::{UserDb, UserId, UserData};
use super::response_with_code;
use crate::index;

pub fn microservice_handler(
    req: Request<Body>,
    user_db: &UserDb,
) -> impl Future<Item = Response<Body>, Error = Error> {

    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {

        (&Method::GET, "/") => {
            *response.body_mut() = index::INDEX.into();
        },

        (method, path) if path.starts_with("/user/") => {
            let user_id = path.trim_start_matches("/user/")
                .parse::<UserId>()
                .ok()
                .map(|x| x as usize);
            let mut users = user_db.lock().unwrap();

            match (method, user_id) {

                (&Method::GET, Some(id)) => {
                    response =
                    if let Some(data) = users.get(id) { // get comes from the Slab type
                        Response::new(data.to_string().into())
                    } else {
                        response_with_code(StatusCode::NOT_FOUND)
                    }
                },

                (&Method::POST, None) => {
                    let id = users.insert(UserData);
                    response = Response::new(id.to_string().into());
                },

                (&Method::POST, Some(_)) => {
                    response = response_with_code(StatusCode::BAD_REQUEST);
                },

                (&Method::PUT, Some(id)) => {
                    response =
                    if let Some(user) = users.get_mut(id) {
                        *user = UserData;
                        response_with_code(StatusCode::OK)
                    } else {
                        response_with_code(StatusCode::NOT_FOUND)
                    }
                },

                (&Method::DELETE, Some(id)) => {
                    response = {
                        if users.contains(id) {
                            users.remove(id);
                            response_with_code(StatusCode::OK)
                        } else {
                            response_with_code(StatusCode::NOT_FOUND)
                        }
                    }
                },

                _ => {
                    response = response_with_code(StatusCode::METHOD_NOT_ALLOWED)
                },
            }
        },

        _ => response = response_with_code(StatusCode::NOT_FOUND),
    };
    future::ok(response)
}