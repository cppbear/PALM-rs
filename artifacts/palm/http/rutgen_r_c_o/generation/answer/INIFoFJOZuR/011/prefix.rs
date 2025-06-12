// Answer 0

#[test]
fn test_from_bytes_options() {
    let input = b"OPTIONS";
    from_bytes(input);
}

#[test]
fn test_from_bytes_connect() {
    let input = b"CONNECT";
    from_bytes(input);
}

