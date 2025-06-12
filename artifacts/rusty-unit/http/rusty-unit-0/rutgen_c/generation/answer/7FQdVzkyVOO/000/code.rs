// Answer 0

#[test]
fn test_iter_mut_empty() {
    let mut map = HeaderMap::with_capacity(10);
    let iter = map.iter_mut();
    assert!(iter.index == 0); // Asserting initial state of iterator
}

#[test]
fn test_iter_mut_single_entry() {
    let mut map = HeaderMap::with_capacity(10);
    map.insert("key1", "value1".to_string());
    
    if let Some(mut entry) = map.get_mut("key1") {
        let mut iter = entry.iter_mut();
        assert_eq!(iter.next().unwrap(), &mut "value1".to_string());
    } else {
        panic!("Entry should exist.");
    }
}

#[test]
fn test_iter_mut_multiple_entries() {
    let mut map = HeaderMap::with_capacity(10);
    map.insert("key1", "value1".to_string());
    map.append("key1", "value2".to_string());
    
    if let Some(mut entry) = map.get_mut("key1") {
        let mut iter = entry.iter_mut();
        assert_eq!(iter.next().unwrap(), &mut "value1".to_string());
        assert_eq!(iter.next().unwrap(), &mut "value2".to_string());
    } else {
        panic!("Entry should exist.");
    }
}

#[test]
fn test_iter_mut_no_entry() {
    let mut map = HeaderMap::with_capacity(10);
    
    if let Some(entry) = map.get_mut("nonexistent") {
        let iter = entry.iter_mut();
        assert!(iter.next().is_none());
    } else {
        // Expected behavior: no entry exists
        assert!(true);
    }
}

