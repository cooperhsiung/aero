use super::response::Response;
use ::futures::future::BoxFuture;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::net::TcpStream;

pub struct Handler {
    pub path: String,
    pub method: String,
    pub func: Arc<dyn HandlerFunc>, // for handler
}

pub struct Midwarer {
    pub path: String,
    pub func: Arc<dyn Middleware>, // for middleware
}

// #[derive(Copy)]
pub struct Context<'x> {
    pub body: &'x str,
    pub path: &'x str,
    pub status: i16,
    pub resp: &'x Response,
    pub method: &'x str,
    pub socket: &'x TcpStream,
    // tail: Vec<Arc<dyn Middleware>>,
    pub(crate) tail: &'x [Arc<dyn Middleware>],
    pub handler: &'x Arc<dyn HandlerFunc>,
}

impl<'a> Context<'a> {
    pub async fn send_text(&mut self, data: String) {
        // self.body = data.as_str();
    }

    pub fn status(&mut self, code: i16) -> &mut Self {
        self.status = code;
        self
    }

    pub async fn next(&mut self) {
        if let Some((current, tail)) = self.tail.split_first() {
            self.tail = tail;
            let next_ctx = Context {
                body: self.body,
                method: self.method,
                path: self.path,
                status: self.status,
                resp: self.resp,
                handler: self.handler,
                tail,
                socket: self.socket,
            };
            current.handle(next_ctx).await;
        } else {
            let hctx = HContext {
                body: self.body.to_string(),
                // resp: self.resp,
                socket: self.socket.clone(),
            };

            println!("-------from there {}", 1);
            self.handler.handle(hctx).await;
            // self.resp.respond(self.body.to_string()).await;
        }
    }
}

#[async_trait]
pub trait Middleware: 'static + Send + Sync {
    /// Asynchronously handle the request, and return a response.
    // async fn handle(&self, ctx: Context, next:  fn(Context) ->  BoxFuture<'static, ()>);
    async fn handle(&self, ctx: Context<'_>) -> ();
}

// #[async_trait]
// impl<'a, F> Middleware for F
// where
//     // F: Send + Sync + 'static +  Fn(Context,  fn(Context) ->  BoxFuture<'static, ()> ) -> BoxFuture<'static, ()>,
//     F: Send + Sync + 'static + FnMut(Context<'_>) -> BoxFuture<'a, ()>,
// {
//     async fn handle(&self, mut ctx: Context<'_>) -> () {
//         // let x = Context {body:ctx.body};
//         // let v = (self)(ctx).await;
//         (self)(ctx).await;
//         // return v;
//     }
// }

pub struct HContext {
    pub body: String,
    // pub resp: Response,
    pub socket: TcpStream,
}

impl HContext {
    pub async fn send_text(&self, data: String) {
        // self.body = data;
        // self.resp.respond(data).await
    }
}

#[async_trait]
pub trait HandlerFunc: 'static + Send + Sync {
    /// Asynchronously handle the request, and return a response.
    // async fn handle(&self, ctx: Context, next:  fn(Context) ->  BoxFuture<'static, ()>);
    async fn handle(&self, ctx: HContext) -> ();
}

#[async_trait]
impl<'a, F> HandlerFunc for F
where
    // F: Send + Sync + 'static +  Fn(Context,  fn(Context) ->  BoxFuture<'static, ()> ) -> BoxFuture<'static, ()>,
    F: Send + Sync + 'static + Fn(HContext) -> BoxFuture<'static, ()>,
{
    async fn handle(&self, ctx: HContext) -> () {
        (self)(ctx).await;
    }
}
