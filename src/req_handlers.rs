use crate::{Request, Response};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
pub enum HandlerMethods {
    Get(String),
    Post(String),
}
pub struct RequestHandlers {
    pub handlers: HashMap<HandlerMethods, Handler>,
    pub not_found: Handler,
}

type Handler = fn(Request, Response);

impl RequestHandlers {
    pub fn new() -> RequestHandlers {
        let handlers = HashMap::new();

        let not_found: Handler = |req, mut res| {
            println!("404: Not-Found. Route handler for {} undefined", req.uri);

            res.text(format!("Route handler for {} undefined", req.uri), 404);
        };

        RequestHandlers {
            handlers,
            not_found,
        }
    }

    pub fn get(&mut self, uri: &'static str, handler: Handler) {
        self.handlers
            .insert(HandlerMethods::Get(String::from(uri)), handler);
    }

    pub fn post(&mut self, uri: &'static str, handler: Handler) {
        self.handlers
            .insert(HandlerMethods::Post(String::from(uri)), handler);
    }
}
