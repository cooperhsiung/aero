use serde::{Deserialize, Serialize};

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
    let mut app = Aero::new("127.0.0.1:3000");

    let mut router = Router::new("/api");
    router.get("/book", |ctx: &mut Context, next: Next| {
        ctx.send_json(Book {
            id: 123,
            name: "asd".to_string(),
            price: 123.3,
        });
        next(ctx);
    });
    app.mount(router);

    app.get("/hello", |ctx: &mut Context, next: Next| {
        ctx.send_text("Hello, world!");
    });

    println!("Listening on http://{}", app.socket_addr);

    tokio::runtime::Builder::new_multi_thread()
        // .worker_threads(6)
        .enable_all()
        .build()
        .unwrap()
        .block_on(app.run());
}
