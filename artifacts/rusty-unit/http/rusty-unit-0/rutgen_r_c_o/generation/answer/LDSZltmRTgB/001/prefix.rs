// Answer 0

#[test]
fn test_delete_with_valid_uri() {
    let uri = "https://www.rust-lang.org/";
    let builder = Request::delete(uri);
}

#[test]
fn test_delete_with_another_valid_uri() {
    let uri = "http://example.com";
    let builder = Request::delete(uri);
}

#[test]
#[should_panic]
fn test_delete_with_empty_uri() {
    let uri = "";
    let builder = Request::delete(uri);
}

#[test]
#[should_panic]
fn test_delete_with_invalid_uri_ftp() {
    let uri = "ftp://invalid";
    let builder = Request::delete(uri);
}

#[test]
#[should_panic]
fn test_delete_with_invalid_uri_htp() {
    let uri = "htp://not.a.uri";
    let builder = Request::delete(uri);
}

#[test]
fn test_delete_with_encoded_uri() {
    let uri = "https://www.example.com/some%20path/";
    let builder = Request::delete(uri);
}

#[test]
fn test_delete_with_long_valid_uri() {
    let uri = "https://www.rust-lang.org/docs/long/path/to/resource/with/lots/of/parameters?query=string&another=value";
    let builder = Request::delete(uri);
}

