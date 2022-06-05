// use crate::httpresponse::HttpResponse;
use super::request::HttpRequest;
use super::response::HttpResponse;
use super::router::Router;
use serde::{Deserialize, Serialize};

use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Debug, Clone)]
pub struct Context<'a> {
    pub req: &'a HttpRequest,
    pub res: &'a HttpResponse<'a>,
    pub body: String,
    pub content_type: String,
}

pub type Next<'a> = &'a mut dyn FnMut(&mut Context);

impl<'a> Context<'a> {
    pub fn send_text(&mut self, content: &'a str) {
        self.body = content.to_string();
        self.content_type = "text/html".to_string()
    }
    pub fn send_json(&mut self, content: impl Serialize) {
        let xs = serde_json::to_string(&content).unwrap();
        self.body = xs;
        self.content_type = "application/json".to_string()
    }
}

#[derive(Clone)]
pub struct Handler {
    pub path: String,
    pub method: String,
    pub func: fn(&mut Context, &mut dyn FnMut(&mut Context)),
}

#[derive(Clone)]
pub struct Aero<'a> {
    pub layers: Vec<Handler>,
    pub socket_addr: &'a str,
}

impl<'a> Aero<'a> {
    pub fn new(port: &'a str) -> Self {
        Aero {
            layers: vec![],
            socket_addr: port,
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(self.socket_addr).await?;
        let layers = Arc::new(self.layers);

        loop {
            // Asynchronously wait for an inbound socket.
            let (mut socket, _) = listener.accept().await?;
            let cop = layers.clone();

            tokio::spawn(async move {
                let mut buf = [0; 1024];

                loop {
                    let n = socket
                        .read(&mut buf)
                        .await
                        .expect("failed to read data from socket");

                    if n == 0 {
                        return;
                    }

                    let req: HttpRequest = String::from_utf8(buf.to_vec()).unwrap().into();
                    // println!("{},{}", req.method, req.path);
                    let mut handlers = vec![];
                    for layer in cop.to_vec() {
                        // println!("{},{}", req.path, elem.path);
                        if req.path.starts_with(layer.path.as_str()) {
                            // println!("{},{}", req.method, elem.method);
                            if req.method == layer.method || layer.method == "ALL" {
                                handlers.push(layer.func)
                            }
                        }
                    }

                    // println!("{}", handlers.len());
                    let mut handler = compose(handlers, 0);
                    let res: HttpResponse = HttpResponse::new("200", None, "", Some("OK".into()));
                    let ctx = &mut Context {
                        req: &req,
                        res: &res,
                        body: "".to_string(),
                        content_type: "".to_string(),
                    };

                    handler(ctx, &mut |ctx| {});

                    let result = ctx.body.as_str();
                    let content_type = ctx.content_type.as_str();
                    // println!("------{} out", result);
                    if result == "" {
                        let res = HttpResponse::new("404", None, "", Some("Not Found".into()));
                        socket
                            .write_all(String::from(res).as_bytes())
                            .await
                            .expect("failed to write data to socket");
                    } else {
                        let res = HttpResponse::new("200", None, content_type, Some(result.into()));
                        socket
                            .write_all(String::from(res).as_bytes())
                            .await
                            .expect("failed to write data to socket");
                    }
                }
            });
        }
    }

    pub fn get(&mut self, path: &'a str, func: fn(&mut Context, &mut dyn FnMut(&mut Context))) {
        &self.layers.push(Handler {
            method: "GET".to_string(),
            path: path.to_string(),
            func: func,
        });
    }

    pub fn post(&mut self, path: &'a str, func: fn(&mut Context, &mut dyn FnMut(&mut Context))) {
        &self.layers.push(Handler {
            method: "POST".to_string(),
            path: path.to_string(),
            func: func,
        });
    }

    pub fn hold(&mut self, path: &'a str, func: fn(&mut Context, &mut dyn FnMut(&mut Context))) {
        &self.layers.push(Handler {
            method: "ALL".to_string(),
            path: path.to_string(),
            func: func,
        });
    }

    pub fn mount(&mut self, router: Router) {
        // add router handler
        for x in router.layers {
            &self.layers.push(Handler {
                method: x.method,
                path: format!("{}{}", router.prefix, x.path),
                func: x.func,
            });
        }
    }
}

type MidwareFn = fn(&mut Context, &mut dyn FnMut(&mut Context));

fn compose(
    midwares: Vec<MidwareFn>,
    i: usize,
) -> impl FnMut(&mut Context, &mut dyn FnMut(&mut Context)) {
    move |ctx: &mut Context, next: &mut dyn FnMut(&mut Context)| {
        let len = midwares.len();
        if len == 0 {
            return next(ctx);
        }
        if i >= len - 1 {
            return midwares[i](ctx, next);
        }
        midwares[i](ctx, &mut |ctx| {
            compose(midwares.clone(), i + 1)(ctx, next);
        });
    }
}
