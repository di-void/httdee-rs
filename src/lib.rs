use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
};

const HTTP_VERSION: &str = "1.1";
const CODE_PAIRS: [(u16, &str); 2] = [(200, "200 OK"), (404, "404 Not-Found")];

pub struct HttDee {
    listener: TcpListener,
    port: u16,
    req_handlers: RequestHandlers,
}

// Request
#[derive(Debug)]
pub struct Request {
    // pub params  -- later
    pub body: Option<String>,
    pub uri: String,
}

// Response
#[derive(Debug)]
pub struct Response<'a> {
    status_codes: &'a HashMap<u16, &'a str>,
    pub stream: &'a mut TcpStream,
}

impl<'a> Response<'a> {
    pub fn _json(&mut self) {
        // return json response
    }

    pub fn text(&mut self, content: String, status: u16) {
        let status = self.status_codes.get(&status).unwrap();

        // format response
        let response = format!("HTTP/{HTTP_VERSION} {status}\r\n\r\n{content}");

        self.stream.write_all(response.as_bytes()).unwrap();
    }
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
                RequestMethods::Get(uri, mut stream) => {
                    let get_handler = self
                        .req_handlers
                        .handlers
                        .get(&HandlerMethods::Get(uri.clone()))
                        .unwrap_or_else(|| nf_handler);

                    let request = Request { uri, body: None };

                    let response = Response {
                        stream: &mut stream,
                        status_codes: &status_codes,
                    };

                    get_handler(request, response);
                }

                RequestMethods::Post(uri, _stream) => println!("POST URI: {:?}", uri),
                _ => println!("HTTP Verb not supported"),
            }
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
enum HandlerMethods {
    Get(String),
    Post(String),
}

pub struct RequestHandlers {
    handlers: HashMap<HandlerMethods, Handler>,
    not_found: Handler,
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

enum RequestMethods {
    Get(String, TcpStream),
    Post(String, TcpStream),
    Other,
}

fn parse_request(mut stream: TcpStream) -> RequestMethods {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut headers = String::new();

    // parse headers
    loop {
        let mut line = String::new();
        let n_bytes = buf_reader.read_line(&mut line).unwrap();

        if n_bytes == 0 {
            break;
        }

        if line == "\r\n" {
            break;
        }

        headers.push_str(&line);
    }

    let content_length = parse_content_length(&headers);

    let mut body = vec![0; content_length];

    if content_length > 0 {
        buf_reader
            .read_exact(&mut body)
            .expect("Something went wrong :(");
    }

    let body = String::from_utf8(body).unwrap();

    println!("Request Body: {}", body);

    // "GET / HTTP/1.1"
    let req_line = headers.lines().next().unwrap();
    let method_uri = req_line.split(' ').take(2).collect::<Vec<_>>();
    let [method, uri] = method_uri[..].try_into().expect("req moving mad :(");

    match method {
        "GET" => RequestMethods::Get(uri.to_string(), stream),
        "POST" => RequestMethods::Post(uri.to_string(), stream),
        _ => RequestMethods::Other,
    }
}

fn parse_content_length(headers: &String) -> usize {
    for line in headers.lines() {
        if line.to_lowercase().starts_with("content-length:") {
            let length = line["content-length:".len()..].trim().parse().unwrap();
            return length;
        }
    }

    0
}
