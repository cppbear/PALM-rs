// Answer 0

#[test]
fn test_from_static_valid_path_and_query() {
    use http::uri::{PathAndQuery, Bytes};

    let valid_path = PathAndQuery::from_static("/hello?world");
    
    assert_eq!(valid_path.path(), "/hello");
    assert_eq!(valid_path.query(), Some("world"));
}

#[test]
#[should_panic]
fn test_from_static_invalid_path() {
    use http::uri::{PathAndQuery};

    // This is expected to panic due to invalid path format
    let _ = PathAndQuery::from_static("invalid-path!");
}

#[test]
fn test_from_static_no_query() {
    use http::uri::{PathAndQuery};

    let no_query_path = PathAndQuery::from_static("/hello");

    assert_eq!(no_query_path.path(), "/hello");
    assert_eq!(no_query_path.query(), None);
}

#[test]
#[should_panic]
fn test_from_static_empty_path() {
    use http::uri::{PathAndQuery};

    // This is expected to panic due to empty path
    let _ = PathAndQuery::from_static("");
}

