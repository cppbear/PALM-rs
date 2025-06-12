// Answer 0

#[test]
fn test_shrink_to_fit_with_large_initial_capacity() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.capacity() >= 100);
    map.shrink_to_fit();
    assert!(map.capacity() >= 2);
}

#[test]
fn test_shrink_to_fit_empty_map() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::new();
    assert!(map.capacity() == 0);
    map.shrink_to_fit();
    assert!(map.capacity() == 0);
}

#[test]
fn test_shrink_to_fit_with_one_element() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(50);
    map.insert(1, 1);
    assert!(map.capacity() >= 50);
    map.shrink_to_fit();
    assert!(map.capacity() >= 1);
}

#[test]
fn test_shrink_to_fit_multiple_elements() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(150);
    for i in 0..10 {
        map.insert(i, i * 10);
    }
    assert!(map.capacity() >= 150);
    map.shrink_to_fit();
    assert!(map.capacity() >= 10);
}

