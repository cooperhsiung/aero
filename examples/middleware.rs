use std::thread;
use std::time::{Duration, Instant};

use aero::Aero;
use aero::Context;
use aero::Next;

fn main() {
    let mut app = Aero::new("127.0.0.1:3000");

    app.hold("/", |ctx: &mut Context, next: Next| {
        println!("middleware start -> {}", ctx.req.path);
        next(ctx);
        println!("middleware end -> {}", ctx.req.path);
    });

    app.hold("/", |ctx: &mut Context, next: Next| {
        let start = Instant::now();
        next(ctx);
        let duration = start.elapsed();
        println!(
            "[access] {} {} cost {:?}ms",
            ctx.req.method,
            ctx.req.path,
            duration.as_millis()
        );
    });

    app.get("/hello", |ctx: &mut Context, next: Next| {
        // some heavy task
        thread::sleep(Duration::from_millis(100));
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
