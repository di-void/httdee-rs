use mime::{self, Mime};
use std::{
    io::{BufReader, Read},
    net::TcpStream,
};

// mime docs: https://docs.rs/mime/latest/mime/

// return String for now
pub fn parse_body(
    reader: &mut BufReader<&mut TcpStream>,
    content_length: usize,
    content_type: Mime,
) -> String {
    match content_type {
        x if x == mime::APPLICATION_JSON => _application_json(reader, content_length),
        x if x == mime::TEXT_PLAIN => _text_plain(reader, content_length),
        x if x == mime::MULTIPART_FORM_DATA => _multipart(reader, content_length),
        _ => _none(),
    }
}

pub fn _none() -> String {
    // unsupported type
    String::from("")
}

pub fn _application_json(reader: &mut BufReader<&mut TcpStream>, content_length: usize) -> String {
    // parse application/json
    let mut body = vec![0; content_length];

    if content_length > 0 {
        reader
            .read_exact(&mut body)
            .expect("Couldn't read body contents :(");
    }

    String::from_utf8(body).unwrap()
}

pub fn _multipart(_reader: &mut BufReader<&mut TcpStream>, _content_length: usize) -> String {
    todo!("not yet implemented");
    // parse multipart/form-data
}

pub fn _text_plain(_reader: &mut BufReader<&mut TcpStream>, _content_length: usize) -> String {
    todo!("not yet implemented");
    // parse text/plain
}
