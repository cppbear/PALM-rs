// Answer 0

#[test]
fn test_try_from_get() {
    let input: &[u8] = b"GET";
    let result = Method::try_from(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Method::GET);
}

#[test]
fn test_try_from_post() {
    let input: &[u8] = b"POST";
    let result = Method::try_from(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Method::POST);
}

#[test]
fn test_try_from_invalid_empty() {
    let input: &[u8] = b"";
    let result = Method::try_from(input);
    assert!(result.is_err());
}

#[test]
fn test_try_from_invalid_method() {
    let input: &[u8] = b"UNKNOWN";
    let result = Method::try_from(input);
    assert!(result.is_err());
}

#[test]
fn test_try_from_put() {
    let input: &[u8] = b"PUT";
    let result = Method::try_from(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Method::PUT);
}

#[test]
fn test_try_from_delete() {
    let input: &[u8] = b"DELETE";
    let result = Method::try_from(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Method::DELETE);
}

#[test]
fn test_try_from_head() {
    let input: &[u8] = b"HEAD";
    let result = Method::try_from(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Method::HEAD);
}

#[test]
fn test_try_from_options() {
    let input: &[u8] = b"OPTIONS";
    let result = Method::try_from(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Method::OPTIONS);
}

#[test]
fn test_try_from_connect() {
    let input: &[u8] = b"CONNECT";
    let result = Method::try_from(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Method::CONNECT);
}

#[test]
fn test_try_from_patch() {
    let input: &[u8] = b"PATCH";
    let result = Method::try_from(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Method::PATCH);
}

#[test]
fn test_try_from_trace() {
    let input: &[u8] = b"TRACE";
    let result = Method::try_from(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Method::TRACE);
}

