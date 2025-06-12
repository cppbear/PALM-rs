// Answer 0

#[test]
fn test_header_map_len_empty() {
    struct TestHeaderValue;
    let map: http::HeaderMap<TestHeaderValue> = http::HeaderMap::with_capacity(0);
    assert_eq!(0, map.len());
}

#[test]
fn test_header_map_len_single_insert() {
    struct TestHeaderValue;
    let mut map: http::HeaderMap<TestHeaderValue> = http::HeaderMap::with_capacity(2);
    map.insert(http::HeaderName::from_static("test-header"), TestHeaderValue);
    assert_eq!(1, map.len());
}

#[test]
fn test_header_map_len_multiple_inserts() {
    struct TestHeaderValue;
    let mut map: http::HeaderMap<TestHeaderValue> = http::HeaderMap::with_capacity(2);
    map.insert(http::HeaderName::from_static("header1"), TestHeaderValue);
    map.insert(http::HeaderName::from_static("header2"), TestHeaderValue);
    assert_eq!(2, map.len());
}

#[test]
fn test_header_map_len_with_append() {
    struct TestHeaderValue;
    let mut map: http::HeaderMap<TestHeaderValue> = http::HeaderMap::with_capacity(2);
    map.insert(http::HeaderName::from_static("header1"), TestHeaderValue);
    map.append(http::HeaderName::from_static("header1"), TestHeaderValue);
    assert_eq!(2, map.len());
}

#[test]
fn test_header_map_len_multiple_appends() {
    struct TestHeaderValue;
    let mut map: http::HeaderMap<TestHeaderValue> = http::HeaderMap::with_capacity(2);
    map.insert(http::HeaderName::from_static("header1"), TestHeaderValue);
    map.append(http::HeaderName::from_static("header1"), TestHeaderValue);
    map.append(http::HeaderName::from_static("header1"), TestHeaderValue);
    assert_eq!(3, map.len());
}

