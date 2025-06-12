// Answer 0

#[test]
fn test_shrink_to_below_current_length() {
    let mut map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    for i in 0..5 {
        map.insert(i, i * 2);
    }
    map.shrink_to(3);
    assert_eq!(map.len(), 3);
}

#[test]
fn test_shrink_to_exact_length() {
    let mut map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    for i in 0..5 {
        map.insert(i, i * 2);
    }
    map.shrink_to(5);
    assert_eq!(map.len(), 5);
}

#[test]
fn test_shrink_to_zero_capacity() {
    let mut map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    for i in 0..5 {
        map.insert(i, i * 2);
    }
    map.shrink_to(0);
    assert_eq!(map.len(), 0);
}

#[test]
fn test_shrink_to_exceeding_capacity() {
    let mut map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    for i in 0..5 {
        map.insert(i, i * 2);
    }
    // This should not panic and should shrink to the minimum specified.
    map.shrink_to(10);
    assert_eq!(map.len(), 5);
}

#[test]
fn test_shrink_to_negative() {
    let mut map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    for i in 0..5 {
        map.insert(i, i * 2);
    }
    // Shrink to a min_capacity that's less than the current length
    map.shrink_to(1);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_shrink_to_empty() {
    let mut map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(0, RandomState::new());
    map.shrink_to(0);
    assert_eq!(map.len(), 0);
}

