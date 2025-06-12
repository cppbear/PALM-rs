// Answer 0

#[test]
fn test_from_bytes_post_method() {
    let input = b"POST";
    let result = http::from_bytes(input);
    assert_eq!(result, Ok(Method(Post)));
}

#[test]
fn test_from_bytes_head_method() {
    let input = b"HEAD";
    let result = http::from_bytes(input);
    assert_eq!(result, Ok(Method(Head)));
}

#[test]
fn test_from_bytes_invalid_method_length() {
    let input = b"NOTA";
    let result = http::from_bytes(input);
    assert!(result.is_err());
}

