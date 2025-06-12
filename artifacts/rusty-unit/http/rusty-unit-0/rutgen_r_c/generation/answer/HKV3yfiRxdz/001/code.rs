// Answer 0

#[test]
fn test_get_all_with_existing_key() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::with_capacity(2);
    map.insert(HOST, "hello".parse().unwrap());
    map.append(HOST, "goodbye".parse().unwrap());

    let view = map.get_all("host");
    
    let mut iter = view.iter();
    assert_eq!(iter.next(), Some(&"hello"));
    assert_eq!(iter.next(), Some(&"goodbye"));
    assert!(iter.next().is_none());
}

#[test]
fn test_get_all_with_non_existing_key() {
    use http::HeaderMap;
    use http::header::USER_AGENT;

    let map = HeaderMap::with_capacity(2);

    let view = map.get_all("user-agent");
    assert!(view.index.is_none());
} 

#[test]
fn test_get_all_with_edge_case_key() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::with_capacity(1);
    map.insert(HOST, "test".parse().unwrap());

    let view = map.get_all("host");
    
    let mut iter = view.iter();
    assert_eq!(iter.next(), Some(&"test"));
    assert!(iter.next().is_none());
} 

#[test]
fn test_get_all_on_empty_map() {
    use http::HeaderMap;

    let map: HeaderMap = HeaderMap::new();
    let view = map.get_all("any-key");
    
    assert!(view.index.is_none());
} 

#[test]
fn test_get_all_with_case_sensitivity() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::with_capacity(2);
    map.insert(HOST, "first-value".parse().unwrap());
    map.append(HOST, "second-value".parse().unwrap());

    let view = map.get_all("HOST"); // testing case sensitivity
    
    let mut iter = view.iter();
    assert_eq!(iter.next(), Some(&"first-value"));
    assert_eq!(iter.next(), Some(&"second-value"));
    assert!(iter.next().is_none());
}

