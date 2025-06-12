// Answer 0

#[test]
fn test_get_all_empty_map() {
    let map: HeaderMap = HeaderMap::with_capacity(10);
    let view = map.get_all("host");
    let iter = view.iter();
    assert!(iter.next().is_none());
}

#[test]
fn test_get_all_single_entry() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("host", "hello".parse().unwrap());
    let view = map.get_all("host");
    let mut iter = view.iter();
    assert_eq!(iter.next().unwrap(), &"hello");
    assert!(iter.next().is_none());
}

#[test]
fn test_get_all_multiple_entries() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("host", "hello".parse().unwrap());
    map.append("host", "goodbye".parse().unwrap());
    let view = map.get_all("host");
    let mut iter = view.iter();
    assert_eq!(iter.next().unwrap(), &"hello");
    assert_eq!(iter.next().unwrap(), &"goodbye");
    assert!(iter.next().is_none());
}

#[test]
#[should_panic]
fn test_get_all_nonexistent_key() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("host", "hello".parse().unwrap());
    let view = map.get_all("nonexistent");
    let iter = view.iter();
    assert!(iter.next().is_none());
}

#[test]
fn test_get_all_after_clear() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("host", "hello".parse().unwrap());
    map.clear();
    let view = map.get_all("host");
    let iter = view.iter();
    assert!(iter.next().is_none());
}

#[test]
fn test_get_all_with_boundary_capacity() {
    let mut map: HeaderMap = HeaderMap::with_capacity(32768);
    map.insert("boundary_host", "edge".parse().unwrap());
    let view = map.get_all("boundary_host");
    let mut iter = view.iter();
    assert_eq!(iter.next().unwrap(), &"edge");
    assert!(iter.next().is_none());
}

