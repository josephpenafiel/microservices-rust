use regex::Regex;
use crate::{Path, Future};
use hyper_staticfile::FileChunkStream;
use hyper::{Request, Response, Body, Method,
StatusCode};
use futures::{future, Stream};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use tokio::fs::File;
type MicroserviceResponse = Box<dyn Future<Item=Response<Body>, Error=std::io::Error> + Send>;

static INDEX: &[u8] = b"Images Microservice";

lazy_static!{
    static ref DOWNLOAD_FILE: Regex = Regex::new("^/download/(?P<filename>\\w{20}?$)").unwrap();
}

pub fn microservice_handler(req: Request<Body>, files: &Path)
-> MicroserviceResponse
{
    match (req.method(), req.uri().path().to_owned().as_ref()) {
        (&Method::GET, "/") => {
            Box::new(future::ok(Response::new(INDEX.into())))
        },
        (&Method::POST, "/upload") => {
            let name: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(20)
                .collect();
            let mut filepath = files.to_path_buf();
            filepath.push(&name);
            let create_file = File::create(filepath);
            let write = create_file.and_then(|file| {
                req.into_body()
                    .map_err(other)
                    .fold(file, |file, chunk| {
                        tokio::io::write_all(file, chunk)
                            .map(|(file, _)| file)
                    })
            });
            let body = write.map(|_| {
                Response::new(name.into())
            });
            Box::new(body)
        },
        (&Method::GET, path) if path.starts_with("/download") => {
            if let Some(cap) = DOWNLOAD_FILE.captures(path) {
                println!("{:?}", cap);
                let filename = cap.name("filename").unwrap().as_str();
                println!("{}", filename);
                let mut filepath = files.to_path_buf();
                filepath.push(filename);
                println!("{:?}", filepath);
                let open_file = File::open(filepath);
                let body = open_file.map(|file| {
                    let chunks = FileChunkStream::new(file);
                    Response::new(Body::wrap_stream(chunks))
                });
                Box::new(body)
            } else {
                response_with_code(StatusCode::NOT_FOUND)
            }
        },
        _ => {
            response_with_code(StatusCode::NOT_FOUND)
        }
    }
}

fn other<E>(err: E) -> std::io::Error
where E: Into<Box<dyn std::error::Error + Send + Sync>>
{
    std::io::Error::new(std::io::ErrorKind::Other, err)
}

fn response_with_code(status: StatusCode)
-> MicroserviceResponse
{
    let resp = Response::builder()
        .status(status)
        .body(Body::empty())
        .unwrap();
    Box::new(future::ok(resp))
}