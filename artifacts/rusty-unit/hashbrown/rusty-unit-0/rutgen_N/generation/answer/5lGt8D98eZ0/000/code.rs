// Answer 0

#[test]
fn test_shrink_to_fit() {
    use hashbrown::HashMap;
    
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.capacity() >= 100);
    
    map.shrink_to_fit();
    assert!(map.capacity() >= 2);
}

#[test]
fn test_shrink_to_fit_empty() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(0);
    assert!(map.capacity() == 0);
    
    map.shrink_to_fit();
    assert!(map.capacity() == 0);
}

#[test]
fn test_shrink_to_fit_one_element() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(10);
    map.insert(1, 2);
    assert!(map.capacity() >= 10);
    
    map.shrink_to_fit();
    assert!(map.capacity() >= 1);
}

