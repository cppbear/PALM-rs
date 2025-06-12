// Answer 0

#[test]
fn test_from_str_options() {
    let method = Method::from_str("OPTIONS");
    assert!(method.is_ok());
    assert_eq!(method.unwrap(), Method(Inner::Options));
}

#[test]
fn test_from_str_get() {
    let method = Method::from_str("GET");
    assert!(method.is_ok());
    assert_eq!(method.unwrap(), Method(Inner::Get));
}

#[test]
fn test_from_str_post() {
    let method = Method::from_str("POST");
    assert!(method.is_ok());
    assert_eq!(method.unwrap(), Method(Inner::Post));
}

#[test]
fn test_from_str_put() {
    let method = Method::from_str("PUT");
    assert!(method.is_ok());
    assert_eq!(method.unwrap(), Method(Inner::Put));
}

#[test]
fn test_from_str_delete() {
    let method = Method::from_str("DELETE");
    assert!(method.is_ok());
    assert_eq!(method.unwrap(), Method(Inner::Delete));
}

#[test]
fn test_from_str_head() {
    let method = Method::from_str("HEAD");
    assert!(method.is_ok());
    assert_eq!(method.unwrap(), Method(Inner::Head));
}

#[test]
fn test_from_str_trace() {
    let method = Method::from_str("TRACE");
    assert!(method.is_ok());
    assert_eq!(method.unwrap(), Method(Inner::Trace));
}

#[test]
fn test_from_str_connect() {
    let method = Method::from_str("CONNECT");
    assert!(method.is_ok());
    assert_eq!(method.unwrap(), Method(Inner::Connect));
}

#[test]
fn test_from_str_patch() {
    let method = Method::from_str("PATCH");
    assert!(method.is_ok());
    assert_eq!(method.unwrap(), Method(Inner::Patch));
}

#[test]
fn test_from_str_invalid() {
    let method = Method::from_str("INVALID_METHOD");
    assert!(method.is_err());
}

