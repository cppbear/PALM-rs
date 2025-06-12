// Answer 0

#[test]
fn test_header_value_debug_sensitive() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b"secret"),
        is_sensitive: true,
    };
    
    let mut output = String::new();
    let result = header_value.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "Sensitive");
}

#[test]
fn test_header_value_debug_empty() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b""),
        is_sensitive: false,
    };
    
    let mut output = String::new();
    let result = header_value.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "\"\"");
}

#[test]
fn test_header_value_debug_with_non_visible_ascii() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b"\x01\x02\x03hello\x00world\""),
        is_sensitive: false,
    };
    
    let mut output = String::new();
    let result = header_value.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "\"\\x1\\x2\\x3hello\\x0world\\\"\"");
}

#[test]
fn test_header_value_debug_with_visible_ascii_but_quote() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b"hello\"world"),
        is_sensitive: false,
    };
    
    let mut output = String::new();
    let result = header_value.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "\"hello\\\"world\"");
}

#[test]
fn test_header_value_debug_with_only_quotes() {
    let header_value = HeaderValue {
        inner: Bytes::from_static(b"\""),
        is_sensitive: false,
    };
    
    let mut output = String::new();
    let result = header_value.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "\"\\\"\"");
}

