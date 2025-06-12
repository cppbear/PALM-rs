// Answer 0

#[test]
fn test_authority_with_valid_input() {
    let uri = http::uri::Builder::new()
        .authority("example.com")
        .build()
        .unwrap();
    assert_eq!(uri.authority().unwrap().as_str(), "example.com");
}

#[test]
fn test_authority_with_edge_case() {
    let uri = http::uri::Builder::new()
        .authority("localhost")
        .build()
        .unwrap();
    assert_eq!(uri.authority().unwrap().as_str(), "localhost");
}

#[test]
#[should_panic]
fn test_authority_with_empty_string() {
    let _ = http::uri::Builder::new()
        .authority("")
        .build()
        .unwrap();
}

#[test]
#[should_panic]
fn test_authority_with_invalid_characters() {
    let _ = http::uri::Builder::new()
        .authority("invalid^chars.com")
        .build()
        .unwrap();
}

