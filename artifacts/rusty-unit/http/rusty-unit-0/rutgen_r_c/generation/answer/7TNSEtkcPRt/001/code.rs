// Answer 0

#[test]
fn test_iter_single_value() {
    let mut map = HeaderMap::new();
    let key = "host";
    map.insert(key.parse().unwrap(), "hello.world".parse().unwrap());
    
    let values = map.get_all(key);
    let mut iter = values.iter();
    
    assert_eq!(&"hello.world", iter.next().unwrap());
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_multiple_values() {
    let mut map = HeaderMap::new();
    let key = "host";
    map.insert(key.parse().unwrap(), "hello.world".parse().unwrap());
    map.append(key.parse().unwrap(), "hello.earth".parse().unwrap());
    
    let values = map.get_all(key);
    let mut iter = values.iter();
    
    assert_eq!(&"hello.world", iter.next().unwrap());
    assert_eq!(&"hello.earth", iter.next().unwrap());
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_no_values() {
    let mut map = HeaderMap::new();
    let key = "host";
    
    let values = map.get_all(key);
    let mut iter = values.iter();
    
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_with_different_keys() {
    let mut map = HeaderMap::new();
    
    let key1 = "host";
    let key2 = "user-agent";
    map.insert(key1.parse().unwrap(), "hello.world".parse().unwrap());
    map.append(key1.parse().unwrap(), "hello.earth".parse().unwrap());
    map.insert(key2.parse().unwrap(), "Mozilla/5.0".parse().unwrap());
    
    let values1 = map.get_all(key1);
    let mut iter1 = values1.iter();
    
    assert_eq!(&"hello.world", iter1.next().unwrap());
    assert_eq!(&"hello.earth", iter1.next().unwrap());
    assert!(iter1.next().is_none());

    let values2 = map.get_all(key2);
    let mut iter2 = values2.iter();
    
    assert_eq!(&"Mozilla/5.0", iter2.next().unwrap());
    assert!(iter2.next().is_none());
}

