// Answer 0

#[test]
fn test_sort_unstable_empty() {
    struct DummySet {
        map: IndexMap<i32, (), RandomState>,
    }
    
    let mut set = DummySet { map: IndexMap::default() };
    set.sort_unstable();
    
    assert!(set.map.is_empty());
}

#[test]
fn test_sort_unstable_single_element() {
    struct DummySet {
        map: IndexMap<i32, (), RandomState>,
    }
    
    let mut set = DummySet { map: IndexMap::default() };
    set.map.insert(3, ());
    set.sort_unstable();
    
    assert_eq!(set.map.len(), 1);
    assert_eq!(set.map.first_key_value(), Some((&3, &())));
}

#[test]
fn test_sort_unstable_multiple_elements() {
    struct DummySet {
        map: IndexMap<i32, (), RandomState>,
    }
    
    let mut set = DummySet { map: IndexMap::default() };
    set.map.insert(5, ());
    set.map.insert(2, ());
    set.map.insert(3, ());
    
    set.sort_unstable();
    
    let keys: Vec<_> = set.map.keys().cloned().collect();
    assert_eq!(keys, vec![2, 3, 5]);
}

#[test]
fn test_sort_unstable_duplicates() {
    struct DummySet {
        map: IndexMap<i32, (), RandomState>,
    }
    
    let mut set = DummySet { map: IndexMap::default() };
    set.map.insert(4, ());
    set.map.insert(1, ());
    set.map.insert(4, ());

    set.sort_unstable();
    
    let keys: Vec<_> = set.map.keys().cloned().collect();
    assert_eq!(keys, vec![1, 4]); // Duplicates should not appear in the sorted set
}

