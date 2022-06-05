## Aero

[![crates.io](https://img.shields.io/crates/v/aero.svg)](https://crates.io/crates/aero)

<p align="center"><img src="logo.png" width="480"/></p>

:rocket: A progressive, idiomatic, and minimalist framework for building Rust HTTP services.

- idiomatic router
- composable middlewares

```Rust
fn main() {
    let mut app = Aero::new("127.0.0.1:3000");

    app.get("/", |ctx: &mut Context, next: Next| {
        ctx.send_text("Hello, world!");
    });

    app.get("/hello", |ctx: &mut Context, next: Next| {
        ctx.send_text("Hello, world!");
    });

    println!("Listening on http://{}", app.socket_addr);

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(app.run());
}
```

wip...

- [x] clean code
- [x] response json
- [x] route
- [ ] body parser
- [ ] file orgnanize
- [ ] api orgnanize
- [ ] publish
- [ ] test

### Installation

Add the following line to your Cargo.toml file:

```
aero = "0.1.3"
```

### Dev

```
cargo fmt -- */**.rs
cargo run
```

### Usage

- #### router

```Rust
let mut app = Aero::new("127.0.0.1:3000");

let mut router = Router::new("/api");
router.get("/book", |ctx: &mut Context, next: Next| {
    ctx.send_text("hello, is's /api/book");
    next(ctx);
});

app.mount(router);

println!("Listening on http://{}", app.socket_addr);
```

- #### json response

```Rust
#[derive(Serialize, Deserialize)]
pub struct Book {
    id: i32,
    name: String,
    price: f32,
}

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
```

- #### middleware

```Rust
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
```

examples are listed at [examples](https://github.com/cooperhsiung/aero/tree/master/examples)

### Benckmark

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

## License

MIT
