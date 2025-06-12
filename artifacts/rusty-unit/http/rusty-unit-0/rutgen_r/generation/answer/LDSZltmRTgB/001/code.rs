// Answer 0

#[test]
fn test_delete_with_valid_uri() {
    let request = delete("https://www.rust-lang.org/")
        .body(())
        .unwrap();
        
    assert_eq!(request.method(), Method::DELETE);
    assert_eq!(request.uri().to_string(), "https://www.rust-lang.org/");
}

#[test]
fn test_delete_with_invalid_uri() {
    let result = std::panic::catch_unwind(|| {
        delete("invalid_uri_string")
            .body(())
            .unwrap();
    });
    
    assert!(result.is_err());
}

#[test]
fn test_delete_with_empty_uri() {
    let result = std::panic::catch_unwind(|| {
        delete("")
            .body(())
            .unwrap();
    });
    
    assert!(result.is_err());
}

#[test]
fn test_delete_with_uri_only() {
    let request = delete("http://example.com")
        .body(())
        .unwrap();
        
    assert_eq!(request.method(), Method::DELETE);
    assert_eq!(request.uri().to_string(), "http://example.com");
}

#[test]
fn test_delete_with_http_uri() {
    let request = delete("http://rust-lang.org")
        .body(())
        .unwrap();
        
    assert_eq!(request.method(), Method::DELETE);
    assert_eq!(request.uri().to_string(), "http://rust-lang.org");
}

#[test]
fn test_delete_with_https_uri() {
    let request = delete("https://rust-lang.org")
        .body(())
        .unwrap();
        
    assert_eq!(request.method(), Method::DELETE);
    assert_eq!(request.uri().to_string(), "https://rust-lang.org");
}

