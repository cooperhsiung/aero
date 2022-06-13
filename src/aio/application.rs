// use crate::httpresponse::HttpResponse;
use super::router::Router;
use crate::request::HttpRequest;
use crate::response::HttpResponse;
use serde::{Deserialize, Serialize};

use futures::future::{BoxFuture, FutureExt};
use std::any::{Any, TypeId};
use std::cell::{RefCell, RefMut};
use std::future;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::time::Instant;
use tokio::{
    self,
    runtime::Runtime,
    sync,
    time::{self, Duration},
};

use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use super::context::{Context, HContext, Handler, HandlerFunc, Middleware, Midwarer};
use super::response::Response;

// #[derive(Clone)]
pub struct Aero {
    // pub layers: Vec<Handler>,
    pub socket_addr: &'static str,
    pub handlers: Arc<Vec<Handler>>,
    pub middlewares: Arc<Vec<Midwarer>>,
}

impl Aero {
    pub fn new(port: &'static str) -> Self {
        Aero {
            // layers: vec![],
            handlers: Arc::new(vec![]),
            middlewares: Arc::new(vec![]),
            socket_addr: port,
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(self.socket_addr).await?;

        loop {
            // Asynchronously wait for an inbound socket.
            let (mut socket, _) = listener.accept().await?;
            let handlers = self.handlers.clone();
            let middlewares = self.middlewares.clone();
            // let layers = Arc::clone(&layers);

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
                    println!("{},{}", req.method, req.path);
                    // let mut handlers = vec![];
                    // for layer in layers.iter() {
                    //     // println!("{},{}", req.path, elem.path);
                    //     if req.path.starts_with(layer.path.as_str()) {
                    //         // println!("{},{}", req.method, elem.method);
                    //         if req.method == layer.method || layer.method == "ALL" {
                    //             // handlers.push(layer.func);
                    //         }
                    //     }
                    // }

                    // println!("{}", handlers.len());
                    // let mut handler = compose(handlers, 0);
                    let res: HttpResponse = HttpResponse::new("200", None, "", Some("OK".into()));
                    let mut ctx = Context {
                        // body: "111".to_string(),
                        body: "111222",
                        // path: "/api/v1/books".to_string(),
                        path: "/api/v1/books",
                        // method: "GET".to_string(),
                        method: "GET",
                        status: 200,
                        tail: &(vec![]),
                        resp: &Response {},
                        handler: &(handlers[0].func), // not found
                    };

                    // handler(ctx, &mut |ctx| {});
                    let mut m = vec![];
                    // let m = Arc::get_mut(&mut vv).expect("Registering middleware is not possible after the Client has been used");
                    // let mmm = Arc::new(app.routers.iter().cloned().collect());
                    for mid in middlewares.iter() {
                        m.push(mid.func.clone());
                        // (ctx.tail).push(mid.router);
                    }
                    ctx.tail = &m[..];

                    let path = ctx.path;
                    let method = ctx.method;
                    println!("----- path {}, {} ", path, method);

                    // trigger
                    if let Some((current, tail)) = ctx.tail.split_first() {
                        ctx.tail = tail;
                        current.handle(ctx).await;
                    };

                    let result = "";
                    let content_type = "";

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

    pub fn with(&mut self, mid: impl Middleware) {
        let m = Arc::get_mut(&mut self.middlewares)
            .expect("Registering middleware is not possible after the Client has been used");
        m.push(Midwarer {
            path: "/".to_string(),
            func: Arc::new(mid),
        })
    }

    pub fn get(&mut self, path: &'static str, handler: impl HandlerFunc) {
        let h = Arc::get_mut(&mut self.handlers)
            .expect("Registering middleware is not possible after the Client has been used");
        h.push(Handler {
            method: "GET".to_string(),
            path: path.to_string(),
            func: Arc::new(handler),
        })
    }

    pub fn post(&mut self, path: &'static str, handler: impl HandlerFunc) {
        let h = Arc::get_mut(&mut self.handlers)
            .expect("Registering middleware is not possible after the Client has been used");
        h.push(Handler {
            method: "POST".to_string(),
            path: path.to_string(),
            func: Arc::new(handler),
        })
    }

    pub fn mount(&mut self, router: Router) {
        // add router handler
        let h = Arc::get_mut(&mut self.handlers)
            .expect("Registering middleware is not possible after the Client has been used");
        for x in router.handlers {
            h.push(Handler {
                method: x.method,
                path: format!("{}{}", router.prefix, x.path),
                func: x.func,
            });
        }

        let m = Arc::get_mut(&mut self.middlewares)
            .expect("Registering middleware is not possible after the Client has been used");
        for x in router.middlewares {
            m.push(Midwarer {
                path: format!("{}{}", router.prefix, x.path),
                func: x.func,
            });
        }
    }
}
