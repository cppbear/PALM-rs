// Answer 0

#[test]
fn test_shrink_to_above_current_capacity() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.allocation_size() >= 100);
    map.shrink_to(120);
    assert!(map.allocation_size() >= 100);
}

#[test]
fn test_shrink_to_exact_capacity() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.allocation_size() >= 100);
    map.shrink_to(100);
    assert!(map.allocation_size() >= 100);
}

#[test]
fn test_shrink_to_below_current_capacity() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.allocation_size() >= 100);
    map.shrink_to(50);
    assert!(map.allocation_size() >= 50);
}

#[test]
fn test_shrink_to_zero_capacity() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.allocation_size() >= 100);
    map.shrink_to(0);
    assert!(map.allocation_size() >= 2);
}

#[test]
fn test_shrink_to_no_change() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.allocation_size() >= 100);
    map.shrink_to(10);
    assert!(map.allocation_size() >= 2);
}

