// Answer 0

#[test]
fn test_is_safe_trace() {
    let method = Method::from_bytes(&[b'T', b'r', b'a', b'c', b'e']).unwrap();
    method.is_safe();
}

#[test]
fn test_is_safe_head() {
    let method = Method::from_bytes(&[b'H', b'e', b'a', b'd']).unwrap();
    method.is_safe();
}

#[test]
fn test_is_safe_get() {
    let method = Method::from_bytes(&[b'G', b'e', b't']).unwrap();
    method.is_safe();
}

#[test]
fn test_is_safe_options() {
    let method = Method::from_bytes(&[b'O', b'p', b't', b'i', b'o', b'n', b's']).unwrap();
    method.is_safe();
}

