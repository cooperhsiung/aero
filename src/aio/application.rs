// use crate::httpresponse::HttpResponse;
use crate::request::HttpRequest;
use crate::response::HttpResponse;
use super::router::Router;
use serde::{Deserialize, Serialize};

use std::any::{Any, TypeId};
use std::cell::{RefCell, RefMut};
use std::future;
use std::future::{Future};
use futures::future::{ BoxFuture, FutureExt};
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

#[derive(Debug, Clone)]
pub struct Context<'a> {
    pub req: &'a HttpRequest,
    pub res: &'a HttpResponse<'a>,
    pub body: String,
    pub content_type: String,
    gonext: bool,
}

// pub type  Next = Arc<dyn for<'a> Fn(&'a mut Context) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>>;
// pub type  Next = Box<dyn   Fn(&mut Context) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>  + Send + Sync + 'static>;
pub type  Next = Box<dyn for<'a> Fn(&'a mut Context) -> Pin<Box<dyn Future<Output = ()> +Send + 'a>>   + Send
+ Sync  >;
// pub type  Next = Box<dyn for<'a> Fn(&'a mut Context) -> BoxFuture<'a,()>>;

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

// unsafe impl<'a> Send for Context<'a> {}
// unsafe impl<'a> Sync for Context<'a> {}


// pub type MidFn = Box<dyn   Fn(& mut Context, Next ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync + 'static>;
// pub type MidFn = Box<dyn for<'a> Fn(&'a mut Context, Next ) -> BoxFuture<'a,()>>;
// pub type MidFn = fn(&mut Context, fn(&mut Context) -> Pin<Box<dyn Future<Output = ()>>> );
pub type MidFn = Box<dyn for<'a> Fn(&'a mut Context, Next ) -> Pin<Box<dyn Future<Output = ()> + Send +'a >> + Send + Sync >;

// #[derive(Clone)]
// #[derive(Debug)]
pub struct Handler  {
    pub path: String,
    pub method: String,
    pub func: MidFn,
}

// #[derive(Clone)]
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
        // let layers = Arc::new(self.layers);
        let layers = Arc::new(self.layers);
        // let ll = vec![];
        // for elem in self.layers {
        //     ll.push(elem);
        // }
        // let ll   = Arc::new(ll);
        // let ll = Arc::new(Mutex::new(ll));

        // let db = Arc::new(Mutex::new(vec![]));
        // for elem in self.layers {
        //     db..push(elem);
        // }
        // let layers = Arc::clone(&layers);

        loop {
            // Asynchronously wait for an inbound socket.
            let (mut socket, _) = listener.accept().await?;
            // let cop = layers.clone();
            let layers = Arc::clone(&layers);

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

                    // println!("{:?}", self.layers.len());

                    // let finished = cop.lock().await;
                    // let array = Arc::try_unwrap(cop);
                    // let array = Arc::try_unwrap(cop).unwrap();
                    // let array = array.into_inner().unwrap();
                    // let array = array.lock().unwrap();
                    // let c = layers.lock().await;
                    // let cop = layers.clone();

                    for layer in layers.iter() {
                        // println!("{},{}", req.path, elem.path);
                        if req.path.starts_with(layer.path.as_str()) {
                            // println!("{},{}", req.method, elem.method);
                            if req.method == layer.method || layer.method == "ALL" {


                                // handlers.push(layer.func);
                            }
                        }
                    }

                    // println!("{}", handlers.len());
                    // let mut handler = compose(handlers, 0);
                    let res: HttpResponse = HttpResponse::new("200", None, "", Some("OK".into()));
                    let ctx = &mut Context {
                        req: &req,
                        res: &res,
                        body: "".to_string(),
                        content_type: "".to_string(),
                        gonext: true,
                    };

                    // handler(ctx, &mut |ctx| {});
                    let mut i = 0;
                    for mid in handlers {
                        println!("---- iiii {}, {}", i ,ctx.gonext);
                        i += 1;
                        if !ctx.gonext {
                            continue;
                        }
                        ctx.gonext = false;
                        let x= mid(ctx, Box::new(|ctx: &mut Context | Box::pin(async  {
                            ctx.gonext = true;
                        })));
                        x.await;
                    }

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

    pub fn get(&mut self, path: &'a str, func: MidFn ) {
        &self.layers.push(Handler {
            method: "GET".to_string(),
            path: path.to_string(),
            func: func,
        });
    }

    pub fn post(&mut self, path: &'a str, func: MidFn) {
        &self.layers.push(Handler {
            method: "POST".to_string(),
            path: path.to_string(),
            func: func,
        });
    }

    pub fn hold(&mut self, path: &'a str, func: MidFn) {
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
