use aero::Aero;
use aero::Context;
use aero::Next;

fn main() {
    let mut app = Aero::new("127.0.0.1:3000");

    app.get("/", |ctx: &mut Context, next: Next| {
        ctx.send_text("Hello, world!");
    });

    app.get("/hello", |ctx: &mut Context, next: Next| async {
        ctx.send_text("Hello, world!");
    });

    app.get("/hello", |ctx: &mut Context, next: Next| async {
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
