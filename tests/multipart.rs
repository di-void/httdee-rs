use httdee_rs::body_parser;
use std::io::{BufReader, Cursor};

// Test basic single-part form data
#[test]
fn test_single_part_form_data() {
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let input_string = format!(
        "--{0}\r\nContent-Disposition: form-data; name=\"username\"\r\n\r\nJohnDoe\r\n--{0}--",
        boundary
    );
    let mut input = Cursor::new(input_string.clone());
    let mut reader = BufReader::new(&mut input); // Bufreader<&mut Cursor<String>>

    let result = body_parser::_multipart(&mut reader, input_string.len(), boundary);

    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert_eq!(parsed, "username:JohnDoe;");
}

// // Test form data with multiple parts
#[test]
fn test_multiple_parts_form_data() {
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let input_string = format!(
        "--{0}\r\nContent-Disposition: form-data; name=\"username\"\r\n\r\nJohnDoe\r\n\
            --{0}\r\nContent-Disposition: form-data; name=\"email\"\r\n\r\njohn.doe@example.com\r\n\
            --{0}--",
        boundary
    );

    let mut input = Cursor::new(input_string.clone());
    let mut reader = BufReader::new(&mut input); // Bufreader<&mut Cursor<String>>

    let result = body_parser::_multipart(&mut reader, input_string.len(), boundary);

    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert_eq!(parsed, "username:JohnDoe;email:john.doe@example.com;");
}

// Test with malformed input
#[test]
fn test_malformed_form_data() {
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let input_string = format!(
        "--{0}\r\nContent-Disposition: form-data; name=\"username\"\r\n\
            Invalid input without proper boundary end",
        boundary
    );
    let mut input = Cursor::new(input_string.clone());
    let mut reader = BufReader::new(&mut input); // Bufreader<&mut Cursor<String>>

    let result = body_parser::_multipart(&mut reader, input_string.len(), boundary);

    assert!(result.is_err());
}

// Test with missing boundary
#[test]
fn test_missing_boundary() {
    let input_string = "Content-Disposition: form-data; name=\"username\"\r\n\r\nJohnDoe";

    let mut input = Cursor::new(input_string);
    let mut reader = BufReader::new(&mut input);

    let result = body_parser::_multipart(&mut reader, input_string.len(), "");

    assert!(result.is_err());
}

// Test form data with file upload
// #[test]
// fn test_file_upload_form_data() {
//     let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
//     let mut input = format!(
//         "{0}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"example.txt\"\r\n\
//             Content-Type: text/plain\r\n\r\n\
//             This is a test file content.\r\n\
//             {0}\r\nContent-Disposition: form-data; name=\"description\"\r\n\r\nFile upload test\r\n\
//             {0}--",
//         boundary
//     );
//     let mut input = Cursor::new(input);
//     let mut reader = BufReader::new(&mut input);

//     todo!("Parser not implemeted yet!");
// }

