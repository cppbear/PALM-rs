// Answer 0

#[test]
fn test_get_with_valid_uri() {
    let uri = "https://www.rust-lang.org/";
    let request = Request::get(uri).body(()).unwrap();
    assert_eq!(request.method(), Method::GET);
    assert_eq!(request.uri().to_string(), uri);
}

#[test]
fn test_get_with_valid_uri_with_path() {
    let uri = "https://www.rust-lang.org/book/";
    let request = Request::get(uri).body(()).unwrap();
    assert_eq!(request.method(), Method::GET);
    assert_eq!(request.uri().to_string(), uri);
}

#[test]
#[should_panic]
fn test_get_with_invalid_uri() {
    let uri = "invalid uri";
    let _: Builder = Request::get(uri);
}

#[test]
fn test_get_with_empty_uri() {
    let uri = "";
    let request = Request::get(uri).body(()).unwrap();
    assert_eq!(request.method(), Method::GET);
    assert_eq!(request.uri().to_string(), uri);
}

#[test]
fn test_get_with_special_character_uri() {
    let uri = "https://www.rust-lang.org/?query=val%20ue";
    let request = Request::get(uri).body(()).unwrap();
    assert_eq!(request.method(), Method::GET);
    assert_eq!(request.uri().to_string(), uri);
}

#[test]
fn test_get_with_uri_with_fragment() {
    let uri = "https://www.rust-lang.org/#section";
    let request = Request::get(uri).body(()).unwrap();
    assert_eq!(request.method(), Method::GET);
    assert_eq!(request.uri().to_string(), uri);
}

