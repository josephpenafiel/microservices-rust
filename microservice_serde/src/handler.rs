#![allow(dead_code)]
use crate::{RngRequest, RngResponse};
use futures::{future, Future, Stream};
use hyper::{Body, Error, Method, Request, Response, StatusCode};
use rand::Rng;
use rand::seq::SliceRandom;
use rand_distr::{Bernoulli, Normal, Uniform};
use std::cmp::{min, max};
use crate::color::Color;

fn handle_request(request: RngRequest) -> RngResponse {
    let mut rng = rand::thread_rng();
    match request {
        RngRequest::Uniform { range } => {
            let value = rng.sample(Uniform::from(range)) as f64;
            RngResponse::Value(value)
        },
        RngRequest::Normal { mean, std_dev } => {
            let value = rng.sample(Normal::new(mean, std_dev).unwrap()) as f64;
            RngResponse::Value(value)
        }
        RngRequest::Bernoulli { p } => {
            let value = rng.sample(Bernoulli::new(p).unwrap()) as i8 as f64;
            RngResponse::Value(value)
        },
        RngRequest::Shuffle { mut data } => {
            data.shuffle(&mut rng);
            RngResponse::Bytes(data)
        },
        RngRequest::Color { from, to } => {
            let red = rng.sample(color_range(from.red, to.red));
            let green = rng.sample(color_range(from.green, to.green));
            let blue = rng.sample(color_range(from.blue, to.blue));
            RngResponse::Color(Color::new(red, green, blue))
        }
    }
}

fn color_range(from: u8, to: u8) -> Uniform<u8> {
    let (from, to) = (min(from, to), max(from, to));
    Uniform::new_inclusive(from, to)
}

pub fn microservice_handler(
    req: Request<Body>,
) -> Box<dyn Future<Item = Response<Body>, Error = Error> + Send> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/random") => {
            let body = req
                .into_body()
                .concat2() // requires future::Stream
                .map(|chunks| {
                    let res = serde_json::from_slice::<RngRequest>(chunks.as_ref()) // deserialize json2enum
                        .map(handle_request)
                        .and_then(|resp| serde_json::to_string(&resp)); // serialize enum2json
                    match res {
                        Ok(body) => Response::new(body.into()),
                        Err(err) => Response::builder()
                            .status(StatusCode::UNPROCESSABLE_ENTITY)
                            .body(err.to_string().into())
                            .unwrap(),
                    }
                });
            Box::new(body)
        },
        _ => {
            let resp = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("not found".into())
                .unwrap();

            Box::new(future::ok(resp))
        },
    }
}
