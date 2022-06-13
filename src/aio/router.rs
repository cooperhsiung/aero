// use super::application::{Context, MidFn};
use super::context::{Handler, HandlerFunc, Middleware, Midwarer};
use std::sync::Arc;

pub struct Router {
    pub prefix: String,
    pub handlers: Vec<Handler>,
    pub middlewares: Vec<Midwarer>,
}

impl Router {
    pub fn new(prefix: &'static str) -> Self {
        Router {
            prefix: prefix.to_string(),
            handlers: vec![],
            middlewares: vec![],
        }
    }
    pub fn with(&mut self, mid: impl Middleware) {
        self.middlewares.push(Midwarer {
            path: "/".to_string(),
            func: Arc::new(mid),
        })
    }

    pub fn get(&mut self, path: &'static str, handler: impl HandlerFunc) {
        self.handlers.push(Handler {
            method: "GET".to_string(),
            path: path.to_string(),
            func: Arc::new(handler),
        })
    }

    pub fn post(&mut self, path: &'static str, handler: impl HandlerFunc) {
        self.handlers.push(Handler {
            method: "POST".to_string(),
            path: path.to_string(),
            func: Arc::new(handler),
        })
    }
}
