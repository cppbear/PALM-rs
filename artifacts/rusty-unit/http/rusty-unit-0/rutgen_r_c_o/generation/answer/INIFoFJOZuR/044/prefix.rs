// Answer 0

#[test]
fn test_from_bytes_post() {
    let input = b"POST";
    let method = Method::from_bytes(input);
}

#[test]
fn test_from_bytes_head() {
    let input = b"HEAD";
    let method = Method::from_bytes(input);
}

