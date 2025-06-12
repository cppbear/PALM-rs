// Answer 0

#[test]
fn test_query_with_valid_data() {
    let valid_bytes = Bytes::from_static(b"/hello/world?key=value&foo=bar");
    let path_and_query = PathAndQuery::from_shared(valid_bytes).unwrap();
    
    assert_eq!(path_and_query.query(), Some("key=value&foo=bar"));
}

#[test]
fn test_query_without_query_string() {
    let valid_bytes = Bytes::from_static(b"/hello/world");
    let path_and_query = PathAndQuery::from_shared(valid_bytes).unwrap();
    
    assert!(path_and_query.query().is_none());
}

#[test]
fn test_query_with_query_end_in_fragment() {
    let valid_bytes = Bytes::from_static(b"/hello/world?key=value#fragment");
    let path_and_query = PathAndQuery::from_shared(valid_bytes).unwrap();
    
    assert_eq!(path_and_query.query(), Some("key=value"));
}

#[test]
fn test_query_with_empty_query() {
    let valid_bytes = Bytes::from_static(b"/hello/world?");
    let path_and_query = PathAndQuery::from_shared(valid_bytes).unwrap();
    
    assert_eq!(path_and_query.query(), Some(""));
}

#[test]
#[should_panic]
fn test_query_with_out_of_bounds_access() {
    let valid_bytes = Bytes::from_static(b"/hello/world?key=value&foo=bar#fragment");
    let mut path_and_query = PathAndQuery::from_shared(valid_bytes).unwrap();
    
    // Manually setting query to a position that is out of bounds.
    path_and_query.query = 1000; // This index does not exist in the data.
    
    let _ = path_and_query.query(); // This should panic
}

