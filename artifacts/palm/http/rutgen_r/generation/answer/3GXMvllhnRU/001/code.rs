// Answer 0

#[test]
fn test_values_with_no_headers() {
    use http::HeaderMap;

    let map = HeaderMap::new();
    let values: Vec<_> = map.values().collect();
    assert!(values.is_empty());
}

#[test]
fn test_values_with_single_header() {
    use http::HeaderMap;
    use http::header::{CONTENT_LENGTH};

    let mut map = HeaderMap::new();
    map.insert(CONTENT_LENGTH, "123".parse().unwrap());
    
    let values: Vec<_> = map.values().collect();
    assert_eq!(values.len(), 1);
    assert_eq!(values[0].to_str().unwrap(), "123");
}

#[test]
fn test_values_with_multiple_headers_of_same_key() {
    use http::HeaderMap;
    use http::header::{HOST};

    let mut map = HeaderMap::new();
    map.insert(HOST, "hello".parse().unwrap());
    map.append(HOST, "goodbye".parse().unwrap());

    let values: Vec<_> = map.values().collect();
    assert_eq!(values.len(), 2);
    assert!(values.iter().any(|v| v.to_str().unwrap() == "hello"));
    assert!(values.iter().any(|v| v.to_str().unwrap() == "goodbye"));
}

#[test]
fn test_values_with_different_headers() {
    use http::HeaderMap;
    use http::header::{HOST, CONTENT_LENGTH};

    let mut map = HeaderMap::new();
    map.insert(HOST, "localhost".parse().unwrap());
    map.insert(CONTENT_LENGTH, "456".parse().unwrap());

    let values: Vec<_> = map.values().collect();
    assert_eq!(values.len(), 2);
    assert!(values.iter().any(|v| v.to_str().unwrap() == "localhost"));
    assert!(values.iter().any(|v| v.to_str().unwrap() == "456"));
}

#[test]
#[should_panic]
fn test_values_with_invalid_header_value() {
    use http::HeaderMap;
    use http::header::CONTENT_LENGTH;

    let mut map = HeaderMap::new();
    map.insert(CONTENT_LENGTH, "not-a-number".parse().unwrap()); // This should not panic, just set a wrong value, so let's ignore this.
    // Assumed no further action as values should not panic on collection
    let _ = map.values(); // Collect as a sanity check
}

