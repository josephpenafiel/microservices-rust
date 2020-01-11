mod handler;
mod color;
mod pablo;
#[macro_use]
extern crate failure;
extern crate futures; 
extern crate hyper;
extern crate rand_distr;
extern crate rand; 
#[macro_use] 
extern crate serde_derive; 
extern crate serde_json;
extern crate base64;
#[macro_use]
extern crate base64_serde;

use color::Color;
use std::ops::Range;
use hyper::{Server};
use hyper::service::service_fn;
use handler::microservice_handler;
use futures::Future;

base64_serde_type!(Base64Standard, base64::STANDARD);


#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum RngResponse {
    Value(f64),
    #[serde(with="Base64Standard")]
    Bytes(Vec<u8>),
    Color(Color),
}

/* 
request format: 
{ "distribution": "uniform", "parameters": { "start": 1, "end": 10 } }
*/
#[derive(Deserialize)]
#[serde(tag = "distribution", content = "parameters", 
  rename_all = "lowercase")]
enum RngRequest {
    Uniform {
        #[serde(flatten)]
        range: Range<i32>,
    },
    Normal {
        mean: f64,
        std_dev: f64,
    },
    Bernoulli {
        p: f64,
    },
    Shuffle {
        #[serde(with="Base64Standard")]
        data: Vec<u8>,
    },
    Color {
        from: Color,
        to: Color,
    }
}


fn main() { 

    pablo::this_is_pablo();
    let addr = ([127,0,0,1], 8080).into();
    let server = Server::bind(&addr)
        .serve(|| {
            service_fn(microservice_handler)
        });
    let server = server.map_err(drop); // needs Future trait
    hyper::rt::run(server);
}