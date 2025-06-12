// Answer 0

#[test]
fn test_response_new_empty_string() {
    let response = Response::new("");
}

#[test]
fn test_response_new_short_string() {
    let response = Response::new("short string");
}

#[test]
fn test_response_new_medium_length_string() {
    let response = Response::new("medium length string example");
}

#[test]
fn test_response_new_long_string() {
    let long_body = "a very long string that exceeds normal expectations".repeat(10);
    let response = Response::new(long_body);
}

