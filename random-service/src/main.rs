use hyper::{Body, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use log::{debug, info, trace, warn};
use dotenv::dotenv;
use std::{env, io::{self, Read}, fs::{File}, net::SocketAddr};
use clap::{crate_authors, crate_description,
crate_name, crate_version, Arg, App};
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
    address: SocketAddr,
}


fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("address")
            .short("a")
            .long("address")
            .value_name("ADDRESS")
            .help("Sets an address i.e: 0.0.0.0:<port>")
            .takes_value(true))
        .get_matches();

    info!("Rand Microservice - v0.1.0");
    trace!("Starting...");

    let config = File::open("microservice.toml")
        .and_then(|mut file| {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            Ok(buffer)
        })
        .and_then(|buffer| {
            toml::from_str::<Config>(&buffer)
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
        })
        .map_err(|err| {
            warn!("Can't read config file: {}", err);
        })
        .ok();

    let addr = matches.value_of("address")
        .map(|s| s.to_owned())
        .or(env::var("ADDRESS").ok())
        .and_then(|addr| addr.parse().ok())
        .or(config.map(|config| config.address))
        .or_else(|| Some(([127,0,0,1], 8080).into()))
        .unwrap();

    debug!("Tying to bind server to {}", addr);
    trace!("Creating service handler...");

    let server = Server::bind(&addr)
        .serve(|| {
            service_fn_ok(|req| {
                trace!("Incoming request is: {:?}", req);
                let rand_byte = rand::random::<u8>();
                debug!("Generated value is {}", rand_byte);
                Response::new(Body::from(rand_byte.to_string()))
            })
        });
    info!("Used address: {}", server.local_addr());
    let server = server.map_err(drop);
    debug!("Run!");
    hyper::rt::run(server);
}