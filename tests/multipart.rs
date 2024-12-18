// use parser;

// Test basic single-part form data
#[test]
fn test_single_part_form_data() {
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let input = format!(
        "{0}\r\nContent-Disposition: form-data; name=\"username\"\r\n\r\nJohnDoe\r\n{0}--",
        boundary
    );

    todo!("Parser not implemeted yet!");

    // let parser = Parser {};
    // let result = parser.parse(&input);
    
    // assert!(result.is_ok());
    // let parsed = result.unwrap();
    // assert_eq!(parsed.get("username"), Some(&"JohnDoe".to_string()));
}

// Test form data with multiple parts
#[test]
fn test_multiple_parts_form_data() {
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let input = format!(
        "{0}\r\nContent-Disposition: form-data; name=\"username\"\r\n\r\nJohnDoe\r\n\
            {0}\r\nContent-Disposition: form-data; name=\"email\"\r\n\r\njohn.doe@example.com\r\n\
            {0}--",
        boundary
    );

    todo!("Parser not implemeted yet!");

    // let parser = Parser {};
    // let result = parser.parse(&input);

    // assert!(result.is_ok());
    // let parsed = result.unwrap();
    // assert_eq!(parsed.get("username"), Some(&"JohnDoe".to_string()));
    // assert_eq!(parsed.get("email"), Some(&"john.doe@example.com".to_string()));
}

// Test form data with file upload
#[test]
fn test_file_upload_form_data() {
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let input = format!(
        "{0}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"example.txt\"\r\n\
            Content-Type: text/plain\r\n\r\n\
            This is a test file content.\r\n\
            {0}\r\nContent-Disposition: form-data; name=\"description\"\r\n\r\nFile upload test\r\n\
            {0}--",
        boundary
    );

    todo!("Parser not implemeted yet!");

    // let parser = Parser {};
    // let result = parser.parse(&input);
    
    // assert!(result.is_ok());
    // let parsed = result.unwrap();
    // assert_eq!(parsed.get("description"), Some(&"File upload test".to_string()));
    // // Note: Actual file handling would be more complex and might require a different return type
}

// Test with malformed input
#[test]
fn test_malformed_form_data() {
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let input = format!(
        "{0}\r\nContent-Disposition: form-data; name=\"username\"\r\n\
            Invalid input without proper boundary end",
        boundary
    );

    todo!("Parser not implemeted yet!");

    // let parser = Parser {};
    // let result = parser.parse(&input);
    
    // assert!(result.is_err());
}

// Test with missing boundary
#[test]
fn test_missing_boundary() {
    let input = "Content-Disposition: form-data; name=\"username\"\r\n\r\nJohnDoe";

    todo!("Parser not implemeted yet!");

    // let parser = Parser {};
    // let result = parser.parse(&input);
    
    // assert!(result.is_err());
}