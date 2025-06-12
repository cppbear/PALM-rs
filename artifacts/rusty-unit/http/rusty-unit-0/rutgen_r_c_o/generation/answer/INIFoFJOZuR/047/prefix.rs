// Answer 0

#[test]
fn test_from_bytes_valid_post() {
    let src = b"POST";
    let _ = Method::from_bytes(src);
}

#[test]
fn test_from_bytes_invalid_post() {
    let src = b"XXXX";
    let _ = Method::from_bytes(src);
} 

#[test]
fn test_from_bytes_empty() {
    let src = b"";
    let _ = Method::from_bytes(src);
}

#[test]
fn test_from_bytes_too_long() {
    let src = b"TOOLONGEXAMPLE";
    let _ = Method::from_bytes(src);
}

