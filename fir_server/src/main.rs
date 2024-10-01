use std::convert::Infallible;
use std::net::SocketAddr;

use fir_server;
use std::sync::mpsc;
use http_body_util::{combinators::BoxBody, BodyExt};
use http_body_util::{Empty, Full};
use hyper::body::Frame;
use hyper::body::{Body, Bytes};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, StatusCode};
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

async fn new_connection(
    _: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    Ok(Response::new(full("connection")))
}

// https://hyper.rs/guides/1/server/echo/
async fn restful_receive(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(full("Try POSTing data to /echo"))),
        (&Method::GET, "/state") => Ok(Response::new(full("Server State is Ok"))),
        (&Method::GET, "/connect") => new_connection(req).await,
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body().boxed())),
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(restful_receive))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
