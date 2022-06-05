use aero::Aero;
use aero::Context;
use aero::Next;
use std::time::Instant;
use tokio::time::{sleep, Duration};
use tokio::{ runtime::Runtime};


fn main() {

    let rt = tokio::runtime::Builder::new_multi_thread()
        // .worker_threads(6)
        .enable_all()
        .build()
        .unwrap();

    let mut app = Aero::new("127.0.0.1:3000" , rt);

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

    app.get("/", |ctx: &mut Context, next: Next| {
        ctx.send_text("Hello, world2!");
        next(ctx)
    });

    app.get("/hello", |ctx: &mut Context, next: Next| {
        // ctx.send_text("Hello, world2!");
        // next(ctx)
        // let rt = app.rt.unwrap();
        // rt.clone().block_on(
        //     async {

        //     }
        // );
       
    });

    println!("Listening on http://{}", app.socket_addr);
    // app.rt = Some(&rt);
    rt.block_on(app.run());
}
