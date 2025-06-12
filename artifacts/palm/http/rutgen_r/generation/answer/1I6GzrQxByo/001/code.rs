// Answer 0

#[test]
fn test_connect_with_valid_uri() {
    let uri = "https://www.rust-lang.org/";
    let builder = connect(uri);
    assert_eq!(builder.method(), Method::CONNECT);
    assert_eq!(builder.uri().to_string(), uri);
}

#[test]
#[should_panic]
fn test_connect_with_invalid_uri() {
    let invalid_uri = "invalid_uri";
    let _builder = connect(invalid_uri);
}

#[test]
fn test_connect_with_empty_uri() {
    let uri = "";
    let builder = connect(uri);
    assert_eq!(builder.method(), Method::CONNECT);
    assert_eq!(builder.uri().to_string(), uri);
}

#[test]
fn test_connect_with_special_characters_uri() {
    let uri = "https://example.com/special_#chars@!";
    let builder = connect(uri);
    assert_eq!(builder.method(), Method::CONNECT);
    assert_eq!(builder.uri().to_string(), uri);
}

#[test]
fn test_connect_with_numeric_uri() {
    let uri = "https://123.456.789.012/";
    let builder = connect(uri);
    assert_eq!(builder.method(), Method::CONNECT);
    assert_eq!(builder.uri().to_string(), uri);
}

