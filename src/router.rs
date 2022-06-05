use super::application::{Context, Handler};
use std::future::Future;

pub struct Router<'a> {
    pub prefix: &'a str,
    pub layers: Vec<Handler>,
}

impl<'a> Router<'a> {
    pub fn new(prefix: &'a str) -> Self {
        Router {
            prefix,
            layers: vec![],
        }
    }

    pub fn get(
        &mut self,
        path: &'a str,
        func: fn(&mut Context, &mut dyn FnMut(&mut Context) -> dyn Future<Output = ()>),
    ) {
        &self.layers.push(Handler {
            method: "GET".to_string(),
            path: path.to_string(),
            func: func,
        });
    }

    pub fn post(
        &mut self,
        path: &'a str,
        func: fn(&mut Context, &mut dyn FnMut(&mut Context) -> dyn Future<Output = ()>),
    ) {
        &self.layers.push(Handler {
            method: "POST".to_string(),
            path: path.to_string(),
            func: func,
        });
    }

    pub fn hold(
        &mut self,
        path: &'a str,
        func: fn(&mut Context, &mut dyn FnMut(&mut Context) -> dyn Future<Output = ()>),
    ) {
        &self.layers.push(Handler {
            method: "ALL".to_string(),
            path: path.to_string(),
            func: func,
        });
    }
}
