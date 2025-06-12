// Answer 0

#[test]
fn test_shrink_to_fit_empty() {
    use hashbrown::HashMap;
    
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.shrink_to_fit();
    assert_eq!(map.allocation_size(), 0);
}

#[test]
fn test_shrink_to_fit_single_element() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    assert!(map.allocation_size() >= 100);
    map.shrink_to_fit();
    assert!(map.allocation_size() >= 1);
}

#[test]
fn test_shrink_to_fit_multiple_elements() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    for i in 0..10 {
        map.insert(i, i * 2);
    }
    assert!(map.allocation_size() >= 100);
    map.shrink_to_fit();
    assert!(map.allocation_size() >= 10);
}

#[test]
fn test_shrink_to_fit_after_removal() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    for i in 0..10 {
        map.insert(i, i * 2);
    }
    assert!(map.allocation_size() >= 100);
    
    map.remove(&5);
    map.shrink_to_fit();
    assert!(map.allocation_size() >= 9);
}

