// Answer 0

#[test]
fn test_path_and_query_valid_input() {
    use http::{self, uri::Builder, PathAndQuery};

    let uri = Builder::new()
        .path_and_query("/hello?foo=bar")
        .build()
        .unwrap();
    
    assert_eq!(uri.path(), "/hello");
    assert_eq!(uri.query(), Some("foo=bar"));
}

#[test]
#[should_panic]
fn test_path_and_query_invalid_input() {
    use http::{self, uri::Builder};

    let _uri = Builder::new()
        .path_and_query("invalid_uri") // This should panic due to invalid input format
        .build()
        .unwrap();
}

#[test]
fn test_path_and_query_edge_case_empty() {
    use http::{self, uri::Builder, PathAndQuery};

    let uri = Builder::new()
        .path_and_query("") // Testing with empty path and query
        .build()
        .unwrap();
    
    assert_eq!(uri.path(), "");
    assert_eq!(uri.query(), None);
}

