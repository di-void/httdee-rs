use mime::{self, Mime};
use std::{
    io::{BufReader, Read, BufRead},
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
        x if x == mime::MULTIPART_FORM_DATA => _multipart(
            reader,
            content_length,
            content_type.get_param(mime::BOUNDARY).unwrap().into(),
        ),
        _ => _none(),
    }
}

// TODO: make parsers a struct and each of
// these functions methods on the Parser struct

pub fn _none() -> Result<String, &'static str> {
    // unsupported type
    Ok(String::from("Unsupported content type :|"))
}

pub fn _application_json<T: Read>(
    reader: &mut BufReader<&mut T>,
    content_length: usize,
) -> Result<String, &'static str> {
    // parse application/json
    let mut body = vec![0; content_length];

    if content_length > 0 {
        reader
            .read_exact(&mut body)
            .expect("Couldn't read body contents :(");
    }

    Ok(String::from_utf8(body).unwrap())
}

pub fn _multipart<T: Read>(
    reader: &mut BufReader<&mut T>,
    content_length: usize,
    _boundary: &str,
) -> Result<String, &'static str> {
    // TODO: refactor this code
    fn parse_headers(line: String, part: &mut String, is_header: &mut bool) -> Result<(), &'static str> {
        let line = line.trim();

        if line.is_empty() {
            *is_header = false;
            return Ok(());
        }

        let lower_line = line.to_lowercase();

        if lower_line.starts_with("content-disposition") && !lower_line.contains("filename") {
            // split line
            let mut line_split = line.split("; ");
            // get rid of left part i.e 'content-disposition'
            line_split.next();

            // extract each header
            for p in line_split {
                let keys = p.split('"').collect::<Vec<&str>>();
                let mut ky = format!("{}", keys[1]);

                // is header name "name="
                if keys[0] == "name=" {
                    ky.push(':');
                }

                part.push_str(&ky);
            }

            Ok(())

        } else if lower_line.starts_with("content-disposition") && lower_line.contains("filename") {
            // handle file content
            Ok(())
        } else {
            Err("something went wrong")
        }
    }

    let boundary = format!("--{}", _boundary);
    let end_boundary = format!("--{}--", _boundary);
    let handle = reader.take(content_length.try_into().unwrap());

    let mut parts = Vec::new();
    let mut current_part = String::new();
    // TODO: consider using read_until to extract headers
    // for separate parsing
    let mut is_header = false;

    for line in handle.lines() {
        let line = line.unwrap();

        if line == boundary {
            if !current_part.is_empty() {
                current_part.push(';');
                parts.push(current_part.clone());
                current_part.clear();
            }

            is_header = true;
        } else if line == end_boundary {
            current_part.push(';');
            parts.push(current_part.clone());
            current_part.clear();
        } else {
            if is_header == true {
                parse_headers(line.to_string(), &mut current_part, &mut is_header)?;
                continue;
            }

            let part_len = current_part.len();
            if part_len == 0 {
                return Err("something went wrong");
            }

            current_part.push_str(&line);
        }
    }

    let parts = parts.into_iter().collect::<String>();

    Ok(parts)
}

pub fn _text_plain<T: Read>(
    _reader: &mut BufReader<&mut T>,
    _content_length: usize,
) -> Result<String, &'static str> {
    todo!("not yet implemented");
    // parse text/plain
}
