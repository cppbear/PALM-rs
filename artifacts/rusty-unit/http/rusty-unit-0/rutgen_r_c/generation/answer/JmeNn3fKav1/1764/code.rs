// Answer 0

#[test]
fn test_from_shared_valid_with_query() {
    let input = Bytes::from_static(b"/some/path?query=value");
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_ok());
    
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.query, 12); // Index of '?' should be 12
}

#[test]
fn test_from_shared_valid_no_query() {
    let input = Bytes::from_static(b"/another/path");
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_ok());
    
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.query, u16::MAX); // No query should set query to u16::MAX
}

#[test]
fn test_from_shared_valid_with_fragment() {
    let input = Bytes::from_static(b"/path#fragment");
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_ok());
    
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.data.bytes, Bytes::from_static(b"/path")); // Data should exclude fragment
}

#[test]
fn test_from_shared_invalid_character() {
    let input = Bytes::from_static(b"/path\xFF"); // Invalid UTF-8 character
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_err());
}

#[test]
fn test_from_shared_invalid_query_character() {
    let input = Bytes::from_static(b"/path?query@\xFF");
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_err());
} 

#[test]
fn test_from_shared_valid_special_characters() {
    let input = Bytes::from_static(b"/some/path/{some_json_data}");
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_ok());
    
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.query, u16::MAX); // No query should set query to u16::MAX
} 

#[test]
fn test_from_shared_empty_string() {
    let input = Bytes::from_static(b"");
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_err());
}

