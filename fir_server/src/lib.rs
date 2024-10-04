mod database;
mod game;
mod game_queue;
mod match_queue;
mod thread_pool;
mod user_info;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;

use http_body_util::{combinators::BoxBody, BodyExt};
use http_body_util::{Empty, Full};
use hyper::body::{self, Frame};
use hyper::body::{Body, Bytes};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, StatusCode};
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use match_queue::UserRegisterData;
use std::sync::mpsc;
use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncBufReadExt, AsyncReadExt};
use tokio::net::TcpListener;

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

async fn handle_connect(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    Ok(Response::new(full("connection")))
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

async fn run_server(
    queue_sender: tokio::sync::mpsc::Sender<match_queue::UserRegisterData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Run Server!");

    // bind ip and make listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    loop {
        // get tcp connection
        let (stream, sddr) = listener.accept().await?;
        println!("connect {sddr:?}");

        let io = TokioIo::new(stream);

        // make service function
        // https://hyper.rs/guides/1/server/echo/
        let service =
            hyper::service::service_fn(move |mut req: Request<body::Incoming>| async move {
                match (req.method(), req.uri().path()) {
                    // check
                    (&Method::GET, "/") => Ok(Response::new(full("Hello, World!"))),

                    // check server state
                    (&Method::GET, "/state") => Ok(Response::new(full("I'm fine"))),

                    // connect user
                    (&Method::GET, "/connect") => {
                        if hyper_tungstenite::is_upgrade_request(&req) {
                            let result = hyper_tungstenite::upgrade(&mut req, None);
                            if let Err(e) = result {
                                eprintln!("{e}");
                                return Ok(Response::new(full("Upgrade fail")));
                            }

                            let (response, socket) = result.unwrap();

                            return Ok(Response::new(full("Upgrade success")));
                        } else {
                            // request is not for protocol upgrade.
                            return Ok(Response::new(full(
                                "Send me an upgrade header with /connect",
                            )));
                        }
                    }

                    // test
                    (&Method::GET, "/echo") => Ok(Response::new(req.into_body().boxed())),
                    _ => {
                        let mut not_found = Response::new(empty());
                        *not_found.status_mut() = StatusCode::NOT_FOUND;
                        // type specifing in async block
                        // https://rust-lang.github.io/async-book/07_workarounds/02_err_in_async_blocks.html
                        Ok::<Response<BoxBody<hyper::body::Bytes, hyper::Error>>, hyper::Error>(
                            not_found,
                        )
                    }
                }
            });

        // handle request
        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service)
                .with_upgrades()
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

/// make threads and run
pub async fn run() {
    println!("Server Start!");

    let mut queue = match_queue::MatchQueue::new();
    let sender = queue.get_sender();

    let server_handle = tokio::spawn(run_server(sender));
    let match_queue_handle = tokio::spawn(match_queue::MatchQueue::run(queue));

    tokio::join!(match_queue_handle, server_handle);
    println!("Server end!");
}
