// Answer 0

#[test]
fn test_reserve_initial_capacity() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, i32> = HashMap::new();
    assert_eq!(map.capacity(), 0);
    map.reserve(10);
    assert!(map.capacity() >= 10);
}

#[test]
fn test_reserve_multiple_times() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.reserve(5);
    assert!(map.capacity() >= 5);
    map.reserve(10);
    assert!(map.capacity() >= 15);
}

#[test]
#[should_panic]
fn test_reserve_exceed_isize_max() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, i32> = HashMap::new();
    // This exceeds isize::MAX and should panic
    map.reserve(usize::MAX);
}

#[test]
fn test_reserve_zero_additional() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.reserve(0);
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_reserve_with_non_zero_initial_capacity() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, i32> = HashMap::with_capacity(10);
    assert!(map.capacity() >= 10);
    map.reserve(5);
    assert!(map.capacity() >= 15);
}

