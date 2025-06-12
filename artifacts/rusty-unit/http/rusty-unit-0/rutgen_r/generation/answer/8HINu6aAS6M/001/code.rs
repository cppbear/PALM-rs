// Answer 0

#[test]
fn test_iter_with_single_value() {
    use http::header::{HeaderMap, Entry, HOST};
    let mut map = HeaderMap::new();
    map.insert(HOST, "single_value".parse().unwrap());
    
    if let Entry::Occupied(e) = map.entry("host") {
        let mut iter = e.iter();
        assert_eq!(&"single_value", iter.next().unwrap());
        assert!(iter.next().is_none());
    }
}

#[test]
fn test_iter_with_multiple_values() {
    use http::header::{HeaderMap, Entry, HOST};
    let mut map = HeaderMap::new();
    map.insert(HOST, "world".parse().unwrap());
    map.append(HOST, "earth".parse().unwrap());
    
    if let Entry::Occupied(e) = map.entry("host") {
        let mut iter = e.iter();
        assert_eq!(&"world", iter.next().unwrap());
        assert_eq!(&"earth", iter.next().unwrap());
        assert!(iter.next().is_none());
    }
}

#[test]
fn test_iter_with_no_values() {
    use http::header::{HeaderMap, Entry, HOST};
    let mut map = HeaderMap::new();
    
    if let Entry::Vacant(_) = map.entry("host") {
        // No values should be associated
        assert!(true); // Just confirming the branch is reached
    }
}

#[test]
#[should_panic]
fn test_iter_on_non_existent_entry() {
    use http::header::{HeaderMap, ENTRY, HOST};
    let mut map = HeaderMap::new();
    map.insert(HOST, "example".parse().unwrap());
    
    // Trying to access a non-existent entry that panics
    let iter = map.entry("non_existent");
    if let Entry::Occupied(_) = iter {
        panic!("This should not panic");
    }
}

