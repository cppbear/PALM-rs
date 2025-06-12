// Answer 0

#[test]
fn test_from_bytes_get() {
    let input = b"GET";
    let result = Method::from_bytes(input);
}

#[test]
fn test_from_bytes_put() {
    let input = b"PUT";
    let result = Method::from_bytes(input);
}

