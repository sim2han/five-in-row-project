use crate::database::data::UserData;
use crate::database::{info, Database, UpdateQuery};
use crate::prelude::*;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::match_queue::UserRegisterData;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::{self, Bytes};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Body, Method, StatusCode};
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;

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
    update_sender: Sender<UpdateQuery>,
    data: Arc<Mutex<Database>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    log("http handle fn starts on 127.0.0.1:3000!");

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
        let queue_sender = queue_sender.clone();
        let update_sender = update_sender.clone();
        let data = data.clone();
        let service = service_fn(move |mut req: Request<body::Incoming>| {
            let queue_sender = queue_sender.clone();
            let update_sender = update_sender.clone();
            let data = data.clone();
            async move {
                match (req.method(), req.uri().path()) {
                    (&Method::GET, "/") => Ok(Response::new(full("Hello, World!"))),

                    // check server state
                    (&Method::GET, "/state") => Ok(Response::new(full("I'm fine"))),

                    (&Method::GET, "/getusers") => {
                        let data = data.lock().await;
                        let data = data.get_all_user_serizlie().unwrap();
                        Ok(Response::new(full(data)))
                    }

                    (&Method::GET, "/getgames") => {
                        let data = data.lock().await;
                        let data = data.get_all_game_serialize().unwrap();
                        Ok(Response::new(full(data)))
                    }

                    (&Method::POST, "/register") => {
                        let body = req.collect().await.unwrap().to_bytes();
                        let body_str = String::from_utf8(body.to_vec()).unwrap();
                        let user_info: info::UserInfo = serde_json::from_str(&body_str).unwrap();
                        update_sender
                            .send(UpdateQuery::UserData(user_info.into()))
                            .await
                            .unwrap();
                        Ok(Response::new(full(body_str)))
                    }

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

                            log(&format!("new websocket {socket:?}"));

                            // send socket to user queue
                            queue_sender
                                .send(UserRegisterData::new(String::from("Alice"), socket))
                                .await
                                .unwrap();

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

                    // default
                    _ => {
                        let mut not_found = Response::new(empty());
                        *not_found.status_mut() = StatusCode::NOT_FOUND;
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
