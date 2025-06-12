// Answer 0

#[test]
fn test_put_with_empty_string_uri() {
    let uri = "";
    let _builder = Request::put(uri);
}

#[test]
fn test_put_with_valid_uri() {
    let uri = "https://www.rust-lang.org/";
    let _builder = Request::put(uri);
}

#[test]
fn test_put_with_invalid_uri() {
    let uri = "htp://example.com";
    let _builder = Request::put(uri);
}

#[test]
fn test_put_with_very_long_uri() {
    let uri = "https://example.com/".to_owned() + &"a".repeat(2040); // almost 2048 characters
    let _builder = Request::put(uri);
}

#[test]
fn test_put_with_unsupported_characters_in_uri() {
    let uri = "https://example.com/space test"; // space is unsupported in the URI
    let _builder = Request::put(uri);
}

