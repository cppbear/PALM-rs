// Answer 0

#[test]
fn test_iter_single_value() {
    use http::header::{HeaderMap, HeaderValue};
    
    let mut map = HeaderMap::with_capacity(2);
    let key = "host".parse::<http::header::HeaderName>().unwrap();
    map.insert(key.clone(), HeaderValue::from_static("world"));

    if let Entry::Occupied(e) = map.entry(key.as_str()).unwrap() {
        let mut iter = e.iter();
        assert_eq!(&HeaderValue::from_static("world"), iter.next().unwrap());
        assert!(iter.next().is_none());
    }
}

#[test]
fn test_iter_multiple_values() {
    use http::header::{HeaderMap, HeaderValue};
    
    let mut map = HeaderMap::with_capacity(2);
    let key = "host".parse::<http::header::HeaderName>().unwrap();
    map.insert(key.clone(), HeaderValue::from_static("world"));
    map.append(key.clone(), HeaderValue::from_static("earth"));

    if let Entry::Occupied(e) = map.entry(key.as_str()).unwrap() {
        let mut iter = e.iter();
        assert_eq!(&HeaderValue::from_static("world"), iter.next().unwrap());
        assert_eq!(&HeaderValue::from_static("earth"), iter.next().unwrap());
        assert!(iter.next().is_none());
    }
}

#[test]
fn test_iter_empty_entry() {
    use http::header::{HeaderMap, HeaderValue};

    let map = HeaderMap::with_capacity(2);
    let key = "host".parse::<http::header::HeaderName>().unwrap();

    if let Err(_) = map.entry(key.as_str()) {
        // Expected, since the entry is not found.
        return;
    }
}

#[test]
#[should_panic]
fn test_iter_on_non_occupied_entry() {
    use http::header::{HeaderMap, HeaderValue};
    
    let mut map = HeaderMap::with_capacity(2);
    let key = "host".parse::<http::header::HeaderName>().unwrap();
    
    // Trying to access an entry that has not been inserted yet,
    // should cause a panic when trying to access the iterator.
    let _iter = map.entry(key.as_str()).unwrap();
}

