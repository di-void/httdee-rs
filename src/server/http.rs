// HTTP parsers, request and response logic

use super::body_parser::parse_body;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

const HTTP_VERSION: f32 = 1.1;

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
    pub(crate) status_codes: &'a HashMap<u16, &'a str>,
    pub stream: TcpStream,
}

impl<'a> Response<'a> {
    pub fn text(&mut self, content: String, status: u16) {
        let status = self.status_codes.get(&status).unwrap();

        // format response
        let response = format!("HTTP/{HTTP_VERSION} {status}\r\n\r\n{content}");

        self.stream.write_all(response.as_bytes()).unwrap();
    }

    // TODO: other response methods e.g json
}

#[derive(Eq, PartialEq, Hash)]
pub enum HttpMethod {
    Get(String),
    Post(String),
}

pub enum RequestMethods {
    Get(String, Option<String>),
    Post(String, Option<String>),
    Other,
}

pub fn parse_request(stream: &mut TcpStream) -> RequestMethods {
    let mut buf_reader = BufReader::new(stream);
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

    let body = parse_body(&mut buf_reader, content_length);
    let body = if body.is_empty() { None } else { Some(body) };

    let [method, uri] = parse_req_line(&headers);

    match method {
        "GET" => RequestMethods::Get(uri.to_string(), body),
        "POST" => RequestMethods::Post(uri.to_string(), body),
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

fn parse_req_line(headers: &String) -> [&str; 2] {
    // "GET / HTTP/1.1"
    let req_line = headers.lines().next().unwrap();
    let method_and_uri = req_line.split(' ').take(2).collect::<Vec<_>>();

    /*
        https://doc.rust-lang.org/book/ch18-02-refutability.html

        why this?

        tldr;
        destructuring is for arrays and not vecs because
        it involves a fixed size to be guaranteed for the pattern to match
    */

    method_and_uri
        .try_into()
        .expect("oops! req line moving mad :(")
}
