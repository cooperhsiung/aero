## Aero

:rocket: A progressive, idiomatic, and minimalist framework for building Rust HTTP services.

wip...

- [x] clean code
- [x] response json
- [x] route
- [ ] body parser
- [ ] file orgnanize
- [ ] api orgnanize
- [ ] publish
- [ ] test

### Install

### Dev

```
cargo fmt -- */**.rs
cargo run
```

### Usage

```Rust
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

#[derive(Serialize, Deserialize)]
pub struct Book {
    id: i32,
    name: String,
    price: f32,
}
```

### benckmark

```
Cooper@CooperdeMBP Rust % autocannon http://127.0.0.1:3000/api/shipping/orders
Running 10s test @ http://127.0.0.1:3000/api/shipping/orders
10 connections


┌─────────┬──────┬──────┬───────┬──────┬─────────┬─────────┬──────┐
│ Stat    │ 2.5% │ 50%  │ 97.5% │ 99%  │ Avg     │ Stdev   │ Max  │
├─────────┼──────┼──────┼───────┼──────┼─────────┼─────────┼──────┤
│ Latency │ 0 ms │ 0 ms │ 0 ms  │ 0 ms │ 0.01 ms │ 0.04 ms │ 8 ms │
└─────────┴──────┴──────┴───────┴──────┴─────────┴─────────┴──────┘
┌───────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%   │ Avg     │ Stdev   │ Min     │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Req/Sec   │ 46431   │ 46431   │ 52799   │ 53887   │ 52400   │ 2055.79 │ 46410   │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Bytes/Sec │ 15.2 MB │ 15.2 MB │ 17.3 MB │ 17.6 MB │ 17.1 MB │ 674 kB  │ 15.2 MB │
└───────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┘

Req/Bytes counts sampled once per second.
# of samples: 10

524k requests in 10.01s, 171 MB read
```