mod server;

pub use server::handler::RequestHandlers;
use server::http::*;
use std::{
    collections::HashMap,
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
};

const CODE_PAIRS: [(u16, &str); 3] = [
    (200, "200 OK"),
    (404, "404 Not-Found"),
    (422, "422 Unprocessable-Content"),
];

pub struct HttDee {
    listener: TcpListener,
    port: u16,
    req_handlers: RequestHandlers,
}

impl HttDee {
    pub fn new(port: u16, req_handlers: RequestHandlers) -> io::Result<HttDee> {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let listener = TcpListener::bind(socket)?;

        Ok(HttDee {
            listener,
            port,
            req_handlers,
        })
    }

    pub fn start(&self) {
        println!("Server is listening on port: {}..", self.port);

        // nf = not_found
        let nf_handler = &self.req_handlers.not_found;
        let status_codes: HashMap<_, _> = CODE_PAIRS.into_iter().collect();

        for stream in self.listener.incoming() {
            // todo: maybe handle errors later
            let stream = stream.unwrap();

            match parse_request(stream) {
                RequestMethods::Get(uri, mut stream, body) => {
                    let get_handler = self
                        .req_handlers
                        .handlers
                        .get(&HttpMethod::Get(uri.clone()))
                        .unwrap_or_else(|| nf_handler);

                    // body is None for now
                    let request = Request { uri, body };

                    let response = Response {
                        stream: &mut stream,
                        status_codes: &status_codes,
                    };

                    get_handler(request, response);
                }

                RequestMethods::Post(uri, mut stream, body) => {
                    let post_handler = self
                        .req_handlers
                        .handlers
                        .get(&HttpMethod::Post(uri.clone()))
                        .unwrap_or_else(|| nf_handler);

                    // body is None for now
                    let request = Request { uri, body };

                    let response = Response {
                        stream: &mut stream,
                        status_codes: &status_codes,
                    };

                    post_handler(request, response);
                }
                _ => println!("HTTP Verb not supported"),
            }
        }
    }
}
