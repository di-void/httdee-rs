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
) -> Result<String, &'static str> {
    let c_type = content_type.essence_str();
    
    match c_type {
        x if x == mime::APPLICATION_JSON => _application_json(reader, content_length),
        x if x == mime::TEXT_PLAIN => _text_plain(reader, content_length),
        x if x == mime::MULTIPART_FORM_DATA => _multipart(reader, content_length, content_type.get_param(mime::BOUNDARY).unwrap().into()),
        _ => _none(),
    }
}

// TODO: make parsers a struct and each of
// these functions methods on the Parser struct

pub fn _none() -> Result<String, &'static str> {
    // unsupported type
    Ok(String::from("Unsupported content type :|"))
}

pub fn _application_json<T: Read>(reader: &mut BufReader<&mut T>, content_length: usize) -> Result<String, &'static str> {
    // parse application/json
    let mut body = vec![0; content_length];

    if content_length > 0 {
        reader
            .read_exact(&mut body)
            .expect("Couldn't read body contents :(");
    }

    Ok(String::from_utf8(body).unwrap())
}

pub fn _multipart<T: Read>(_reader: &mut BufReader<&mut T>, _content_length: usize, boundary: &str) -> Result<String, &'static str> {
    // parse multipart/form-data
    println!("Boundary: {}", boundary);

    Ok(String::from("Some MultiPart"))
}

pub fn _text_plain<T: Read>(_reader: &mut BufReader<&mut T>, _content_length: usize) -> Result<String, &'static str> {
    todo!("not yet implemented");
    // parse text/plain
}
