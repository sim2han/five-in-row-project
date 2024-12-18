use crate::database::data::UserData;
use crate::database::info::UserKeyInfo;
use crate::database::{info, Database, UpdateQuery};
use crate::prelude::*;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::match_queue::UserRegisterData;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::{self, Bytes};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, StatusCode};
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::collections::HashMap;
use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use url::Url;

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
    _update_sender: Sender<UpdateQuery>,
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
        let data = data.clone();
        let service = service_fn(move |mut req: Request<body::Incoming>| {
            let queue_sender = queue_sender.clone();
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
                        log(&format!("register {body_str}"));
                        let user_info: info::RegisterInfo =
                            serde_json::from_str(&body_str).unwrap();
                        let mut data = data.lock().await;
                        let key = data.register_user(user_info);
                        let response = serde_json::to_string(&key).unwrap();
                        Ok(Response::new(full(response)))
                    }

                    (&Method::POST, "/login") => {
                        let body = req.collect().await.unwrap().to_bytes();
                        let body_str = String::from_utf8(body.to_vec()).unwrap();
                        log(&format!("login {body_str}"));
                        let info: info::LoginInfo = serde_json::from_str(&body_str).unwrap();
                        let data = data.lock().await;
                        let user = data.try_login(&info);
                        let resp = match user {
                            Some(user) => {
                                serde_json::to_string::<info::UserKeyInfo>(&info::UserKeyInfo {
                                    key: user.key.clone(),
                                })
                                .unwrap()
                            }
                            None => {
                                serde_json::to_string::<info::UserKeyInfo>(&info::UserKeyInfo {
                                    key: "".to_string(),
                                })
                                .unwrap()
                            }
                        };
                        Ok(Response::new(full(resp)))
                    }

                    (&Method::GET, "/getuserinfo") => {
                        let body = req.collect().await.unwrap().to_bytes();
                        let body_str = String::from_utf8(body.to_vec()).unwrap();
                        log(&format!("getuserinfo {body_str}"));
                        let key: info::UserKeyInfo = serde_json::from_str(&body_str).unwrap();
                        let data = data.lock().await;
                        let user_data = data.get_user(&key);
                        if let Some(user_data) = user_data {
                            let resp = serde_json::to_string(&user_data).unwrap();
                            Ok(Response::new(full(resp)))
                        } else {
                            let resp = serde_json::to_string(&info::UserInfo {
                                id: String::new(),
                                pwd: String::new(),
                                rating: 0,
                                key: String::new(),
                            })
                            .unwrap();
                            Ok(Response::new(full(resp)))
                        }
                    }

                    (&Method::GET, "/getgameinfo") => {
                        let body = req.collect().await.unwrap().to_bytes();
                        let body_str = String::from_utf8(body.to_vec()).unwrap();
                        log(&format!("getgameinfo {body_str}"));
                        let key: info::UserKeyInfo = serde_json::from_str(&body_str).unwrap();
                        let data = data.lock().await;
                        let games = data.get_user_game(&key);
                        let resp = serde_json::to_string(&games).unwrap();
                        Ok(Response::new(full(resp)))
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

                            // parse url and get key value
                            let url =
                                Url::parse(&format!("ws://localhost:{}", &req.uri().to_string()))
                                    .expect("Failed to parse");
                            let params: HashMap<String, String> = url
                                .query_pairs()
                                .map(|(k, v)| (k.into_owned(), v.into_owned()))
                                .collect();
                            let key = params.get("key").unwrap().clone();

                            let user;
                            if key == "" {
                                user = UserData {
                                    id: String::from("Anonymous"),
                                    pwd: String::from(""),
                                    rating: 0,
                                    key: String::from(""),
                                }
                            } else {
                                let data = data.lock().await;
                                user = data.get_user(&UserKeyInfo { key: key }).unwrap();
                            }

                            let (response, socket) = result.unwrap();
                            let socket = socket;

                            log(&format!("new websocket {socket:?}"));

                            // send socket to user queue
                            queue_sender
                                .send(UserRegisterData::new(user, socket))
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
