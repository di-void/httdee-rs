// request handlers

use super::http::{Request, Response, HttpMethod};
use std::collections::HashMap;

// handler method
type Handler = fn(Request, Response);

pub struct RequestHandlers {
    pub handlers: HashMap<HttpMethod, Handler>,
    pub not_found: Handler,
}

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
            .insert(HttpMethod::Get(String::from(uri)), handler);
    }

    pub fn post(&mut self, uri: &'static str, handler: Handler) {
        self.handlers
            .insert(HttpMethod::Post(String::from(uri)), handler);
    }
}