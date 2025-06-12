// Answer 0

#[test]
fn test_from_bytes_get() {
    let input: &[u8] = b"GET";
    let _ = Method::from_bytes(input);
}

#[test]
fn test_from_bytes_put() {
    let input: &[u8] = b"PUT";
    let _ = Method::from_bytes(input);
}

#[test]
fn test_from_bytes_invalid() {
    let input: &[u8] = b"ABC";
    let _ = Method::from_bytes(input);
}

