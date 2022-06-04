// extern crate aero;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

// use aero::{ Aero };
use aero::Aero;
use aero::Context;
use aero::Next;
use aero::Router;

#[derive(Serialize, Deserialize)]
pub struct Book {
    id: i32,
    name: String,
    price: f32,
}

fn main() {
    println!("Hello, world!");
    let mut app = Aero::new("127.0.0.1:3000");

    let mut router = Router::new("/api");
    router.get("/book", |ctx: &mut Context, next: Next| {
        println!("hold hello - {:?}", ctx.req.path);
        // ctx.setBody("It is book api");
        ctx.send_json(Book {
            id: 123,
            name: "asd".to_string(),
            price: 123.3,
        });
        next(ctx);
    });
    app.mount(router);

    app.hold("/hello", |ctx: &mut Context, next: Next| {
        println!("hold hello - {:?}", ctx.req.path);
        ctx.send_text("xxxxxxxxx");
        next(ctx);
    });

    app.get("/hello", |ctx: &mut Context, next: Next| {
        println!("get hello - {:?}", ctx.req.path);
        ctx.body = "hello world".to_string();
        next(ctx);
    });

    app.get("/hello2", |ctx: &mut Context, next: Next| {
        ctx.send_text("hello world 2");
        next(ctx);
    });

    println!("Listening on http://{}", app.socket_addr);

    tokio::runtime::Builder::new_multi_thread()
        // .worker_threads(6)
        .enable_all()
        .build()
        .unwrap()
        .block_on(app.run());
}

#[test]
fn start() {
    main();
}
