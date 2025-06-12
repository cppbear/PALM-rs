// Answer 0

#[test]
fn test_get_all_with_single_value() {
    use http::HeaderMap;
    use http::header::HOST;
    
    let mut map = HeaderMap::new();
    map.insert(HOST, "hello".parse().unwrap());
    
    let view = map.get_all("host");
    
    let mut iter = view.iter();
    assert_eq!(&"hello", iter.next().unwrap());
    assert!(iter.next().is_none());
}

#[test]
fn test_get_all_with_multiple_values() {
    use http::HeaderMap;
    use http::header::HOST;
    
    let mut map = HeaderMap::new();
    map.insert(HOST, "hello".parse().unwrap());
    map.append(HOST, "goodbye".parse().unwrap());
    
    let view = map.get_all("host");
    
    let mut iter = view.iter();
    assert_eq!(&"hello", iter.next().unwrap());
    assert_eq!(&"goodbye", iter.next().unwrap());
    assert!(iter.next().is_none());
}

#[test]
fn test_get_all_with_no_values() {
    use http::HeaderMap;
    
    let map = HeaderMap::new();
    
    let view = map.get_all("host");
    
    let mut iter = view.iter();
    assert!(iter.next().is_none());
}

#[test]
fn test_get_all_with_case_insensitive_key() {
    use http::HeaderMap;
    use http::header::HOST;
    
    let mut map = HeaderMap::new();
    map.insert(HOST, "hello".parse().unwrap());
    map.append(HOST, "goodbye".parse().unwrap());
    
    let view = map.get_all("HOST");
    
    let mut iter = view.iter();
    assert_eq!(&"hello", iter.next().unwrap());
    assert_eq!(&"goodbye", iter.next().unwrap());
    assert!(iter.next().is_none());
}

