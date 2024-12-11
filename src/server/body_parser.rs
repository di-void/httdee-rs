use std::{
    io::{BufReader, Read},
    net::TcpStream,
};

// return String for now
pub fn parse_body(reader: &mut BufReader<&mut TcpStream>, content_length: usize) -> String {
    let mut body = vec![0; content_length];

    if content_length > 0 {
        reader
            .read_exact(&mut body)
            .expect("Couldn't read body contents :(");
    }

    String::from_utf8(body).unwrap()

    // _multipart()
}

pub fn _multipart() {
    // parse multi-part
}
