// Answer 0

#[test]
fn test_iter_empty_map() {
    let map = HeaderMap::new();
    let values = map.get_all("host");
    let mut iter = values.iter();
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_single_entry() {
    use http::header::HOST;
    
    let mut map = HeaderMap::new();
    map.insert(HOST, "hello.world".parse().unwrap());
    
    let values = map.get_all("host");
    let mut iter = values.iter();
    assert_eq!(&"hello.world", iter.next().unwrap());
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_multiple_entries() {
    use http::header::HOST;
    
    let mut map = HeaderMap::new();
    map.insert(HOST, "hello.world".parse().unwrap());
    map.append(HOST, "hello.earth".parse().unwrap());

    let values = map.get_all("host");
    let mut iter = values.iter();
    assert_eq!(&"hello.world", iter.next().unwrap());
    assert_eq!(&"hello.earth", iter.next().unwrap());
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_case_insensitive() {
    use http::header::HOST;
    
    let mut map = HeaderMap::new();
    map.insert(HOST, "example.com".parse().unwrap());
    map.insert("Host", "example.org".parse().unwrap());

    let values = map.get_all("host");
    let mut iter = values.iter();
    assert_eq!(&"example.com", iter.next().unwrap());
    assert_eq!(&"example.org", iter.next().unwrap());
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_with_non_existing_key() {
    let map = HeaderMap::new();
    let values = map.get_all("non-existing-key");
    let mut iter = values.iter();
    assert!(iter.next().is_none());
}

