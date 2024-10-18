use crate::database::RealData;
use crate::prelude::*;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::match_queue::UserRegisterData;
use http_body_util::{combinators::BoxBody, BodyExt};
use http_body_util::{Empty, Full};
use hyper::body::{self, Bytes};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, StatusCode};
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;

/*
async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
*/

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

pub async fn run_server(
    queue_sender: Sender<crate::match_queue::UserRegisterData>,
    data: Arc<Mutex<RealData>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    log("http handle fn start!");

    // bind ip and make listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    loop {
        // get tcp connection
        let (stream, sddr) = listener.accept().await?;
        log(format!("http request from {sddr:?}").as_str());

        let io = TokioIo::new(stream);

        // make service function
        // https://hyper.rs/guides/1/server/echo/
        // make copy and move sender and data
        let sender = queue_sender.clone();
        let data = data.clone();
        let service = service_fn(move |mut req: Request<body::Incoming>| {
            let sender = sender.clone();
            let data = data.clone();
            async move {
                match (req.method(), req.uri().path()) {
                    (&Method::GET, "/") => Ok(Response::new(full("Hello, World!"))),

                    // check server state
                    (&Method::GET, "/state") => Ok(Response::new(full("I'm fine"))),

                    // connect user with websocket request
                    // https://crates.io/crates/hyper-tungstenite
                    (&Method::GET, "/connect") => {
                        if hyper_tungstenite::is_upgrade_request(&req) {
                            let result = hyper_tungstenite::upgrade(&mut req, None);
                            if let Err(e) = result {
                                log(&format!("fail in upgrade: {e}"));
                                return Ok(Response::new(full("Upgrade fail")));
                            }

                            let (response, socket) = result.unwrap();
                            let socket = socket;

                            // send socket to user queue
                            if let Err(e) = sender.send(UserRegisterData::new(socket)).await {
                                log(format!("Error: {e}").as_str());
                            };

                            // map websocket response to Response<BoxBody<..>>
                            let mut res = Response::new(
                                response
                                    .body()
                                    .clone()
                                    .map_err(|never| match never {})
                                    .boxed(),
                            );
                            *res.status_mut() = response.status();
                            *res.headers_mut() = response.headers().clone();
                            Ok(res)
                        } else {
                            // request is not for protocol upgrade.
                            Ok(Response::new(full("Send me a corret header")))
                        }
                    }

                    // database access test
                    (&Method::GET, "/getall") => {
                        let data = data.lock().await;
                        let data = data.get_all_user_serizlie().unwrap();
                        Ok(Response::new(full(data)))
                    }

                    // default
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
            }
        });

        // handle request
        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service)
                .with_upgrades()
                .await
            {
                log(&format!("Error serving connection: {:?}", err));
            }
        });
    }
}
