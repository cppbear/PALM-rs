// Answer 0

#[test]
fn test_into_mut_valid_entry() {
    use http::header::{HeaderMap, HeaderName, HeaderValue};

    let mut map = HeaderMap::default();
    let header_name = HeaderName::from_static("host");
    map.insert(header_name.clone(), HeaderValue::from_static("hello.world".to_string()));
    map.append(header_name.clone(), HeaderValue::from_static("hello.earth".to_string()));

    if let http::header::Entry::Occupied(entry) = map.entry(header_name) {
        let value = entry.into_mut();
        value.push_str("-2");
    }

    assert_eq!(map[header_name], HeaderValue::from_static("hello.world-2"));
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_into_mut_no_values_panic() {
    use http::header::{HeaderMap, HeaderName};

    let mut map = HeaderMap::default();
    let header_name = HeaderName::from_static("empty-header");

    if let http::header::Entry::Occupied(entry) = map.entry(header_name) {
        entry.into_mut(); // This should panic because there are no values.
    }
}

#[test]
fn test_into_mut_multiple_values() {
    use http::header::{HeaderMap, HeaderName, HeaderValue};

    let mut map = HeaderMap::default();
    let header_name = HeaderName::from_static("content-type");
    map.insert(header_name.clone(), HeaderValue::from_static("text/html".to_string()));
    map.append(header_name.clone(), HeaderValue::from_static("application/json".to_string()));

    if let http::header::Entry::Occupied(entry) = map.entry(header_name) {
        let value = entry.into_mut();
        value.push_str("; charset=utf-8");
    }

    assert_eq!(map[header_name], HeaderValue::from_static("text/html;charset=utf-8"));
}

