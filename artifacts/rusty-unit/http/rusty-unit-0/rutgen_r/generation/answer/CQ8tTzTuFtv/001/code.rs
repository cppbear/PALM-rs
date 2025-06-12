// Answer 0

#[test]
fn test_iter_empty_map() {
    use http::HeaderMap;
    let map = HeaderMap::new();
    let iter = map.iter();
    assert_eq!(iter.entry, 0);
    assert!(iter.cursor.is_none());
}

#[test]
fn test_iter_single_entry() {
    use http::HeaderMap;
    use http::header::{CONTENT_LENGTH, HOST};
  
    let mut map = HeaderMap::new();
    map.insert(HOST, "hello".parse().unwrap());
    
    let mut iter = map.iter();
    assert_eq!(iter.entry, 0);
    assert!(iter.cursor.is_some());
}

#[test]
fn test_iter_multiple_entries_same_key() {
    use http::HeaderMap;
    use http::header::HOST;
    
    let mut map = HeaderMap::new();
    map.insert(HOST, "hello".parse().unwrap());
    map.append(HOST, "goodbye".parse().unwrap());
    
    let mut iter = map.iter();
    assert_eq!(iter.entry, 0);
    assert!(iter.cursor.is_some());
}

#[test]
fn test_iter_multiple_keys() {
    use http::HeaderMap;
    use http::header::{CONTENT_LENGTH, HOST};
    
    let mut map = HeaderMap::new();
    map.insert(HOST, "hello".parse().unwrap());
    map.append(HOST, "goodbye".parse().unwrap());
    map.insert(CONTENT_LENGTH, "123".parse().unwrap());
    
    let mut iter = map.iter();
    assert_eq!(iter.entry, 0);
    assert!(iter.cursor.is_some());
}

#[test]
fn test_iter_with_no_keys() {
    use http::HeaderMap;
    let map = HeaderMap::new();
    {
        let mut iter = map.iter();
        assert_eq!(iter.entry, 0);
        assert!(iter.cursor.is_none());
    }
}

