use aero::Aero;
use aero::Context;
use aero::Next;
use aero::Router;

fn main() {
    let mut app = Aero::new("127.0.0.1:3000");

    let mut router = Router::new("/api");
    router.get("/book", |ctx: &mut Context, next: Next| {
        ctx.send_text("hello, is's /api/book");
        next(ctx);
    });

    app.mount(router);

    println!("Listening on http://{}", app.socket_addr);

    tokio::runtime::Builder::new_multi_thread()
        // .worker_threads(6)
        .enable_all()
        .build()
        .unwrap()
        .block_on(app.run());
}
