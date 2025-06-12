// Answer 0

fn setup() -> Bytes {
    Bytes::from_static(b"%E0%A4%A4%E0%A4%A6#fragment")
}

#[test]
fn test_invalid_uri_with_non_utf8() {
    let src = setup();
    let result = from_shared(src);
    assert!(result.is_err());
}

#[test]
fn test_valid_uri_with_query_and_fragment() {
    let src = Bytes::from_static(b"/path?query=value#fragment");
    let result = from_shared(src);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.query, 5u16); // query starts after "/path"
    assert_eq!(&path_and_query.data, "/path"); // Extracted path without query
}

#[test]
#[should_panic]
fn test_invalid_uri_that_should_panic() {
    let src = Bytes::from_static(b"/path#fragment\xFF");
    let _result = from_shared(src); // Should panic due to invalid UTF-8
}

#[test]
fn test_uri_with_query_and_fragment_with_utf8_chars() {
    let src = Bytes::from_static(b"/path?value=%E2%9C%93#fragment");
    let result = from_shared(src);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.query, 5u16);
    assert_eq!(&path_and_query.data, "/path");
}

#[test]
fn test_uri_with_only_fragment() {
    let src = Bytes::from_static(b"/path#");
    let result = from_shared(src);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.query, NONE); // No query present
    assert_eq!(&path_and_query.data, "/path"); // Extracted path without query
}

#[test]
fn test_uri_with_special_character_handling() {
    let src = Bytes::from_static(b"/path?value=\"{\"val\":\"test\"}#fragment");
    let result = from_shared(src);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.query, 5u16);
    assert_eq!(&path_and_query.data, "/path");
}

