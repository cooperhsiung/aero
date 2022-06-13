use ::futures::future::FutureExt;
use aero::aio::application::Aero;
use aero::aio::context::{Context, HContext, Middleware};
use aero::aio::router::Router;
use async_trait::async_trait;
// // use aero::Context;
// // use aero::Next;

struct TestMid {}
impl TestMid {
    fn new() -> Self {
        TestMid {}
    }
    async fn publish(&self, ctx: Context<'_>) {}
}

#[async_trait]
impl Middleware for TestMid {
    async fn handle(&self, mut ctx: Context<'_>) -> () {
        println!("------ mid start");
        ctx.next().await;
        println!("------ mid end");
    }
}

struct TestMid2 {}
#[async_trait]
impl Middleware for TestMid2 {
    async fn handle(&self, mut ctx: Context<'_>) -> () {
        println!("------ mid2 start");
        ctx.next().await;
        println!("------ mid2 end");
    }
}

fn main() {
    let mut app = Aero::new("127.0.0.1:3000");

    app.with(TestMid::new());
    app.with(TestMid2 {});

    #[rustfmt::skip]
    app.get("/api/v1/books", |ctx: HContext| async move {
        println!("----- handler {}", ctx.body);
        // sleep2().await;
        ctx.send_text("asdasd".to_string()).await;
    }.boxed());

    let mut router = Router::new("/test");
    router.get("/", |ctx: HContext| {
        async move {
            println!("----- handler {}", ctx.body);
            // sleep2().await;
            ctx.send_text("test hanler".to_string()).await;
        }
        .boxed()
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
