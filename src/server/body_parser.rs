use std::{
    io::{BufReader, Read},
    net::TcpStream,
};
use mime::{self, Mime};

// identify the content-type of the body
// this info will be used to determine which
// parsing function will be used organized using pattern matching

// mime docs: https://docs.rs/mime/latest/mime/

// return String for now
pub fn parse_body(reader: &mut BufReader<&mut TcpStream>, content_length: usize, content_type: Mime) -> String {
    match (content_type.type_(), content_type.subtype()) {
        (mime::APPLICATION, mime::JSON) => _application_json(reader, content_length),
        (mime::TEXT, mime::PLAIN) => _text_plain(reader, content_length),
        (mime::MULTIPART, mime::FORM_DATA) => _multipart(reader, content_length),
        _ => _none()
    }
}

pub fn _none() -> String {
    // unsupported type
    String::from("Unknown Content-Type")
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

pub fn _text_plain(_reader: &mut BufReader<&mut TcpStream>, _content_length: usize) -> String {
    todo!("not yet implemented");
    // parse text/plain
}

pub fn _multipart(_reader: &mut BufReader<&mut TcpStream>, _content_length: usize) -> String {
    todo!("not yet implemented");
    // parse multi-part
}
