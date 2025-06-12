// Answer 0

#[test]
fn test_try_reserve_success() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, isize> = HashMap::new();
    assert_eq!(map.capacity(), 0);

    map.try_reserve(10).expect("Failed to reserve capacity");
    assert!(map.capacity() >= 10);
}

#[test]
#[should_panic]
fn test_try_reserve_capacity_overflow() {
    use hashbrown::HashMap;
    use hashbrown::TryReserveError;

    let mut map: HashMap<i32, i32> = HashMap::new();

    match map.try_reserve(usize::MAX) {
        Err(error) => match error {
            TryReserveError::CapacityOverflow => {}
            _ => panic!("Expected TryReserveError::CapacityOverflow"),
        },
        _ => panic!("Expected an error but got Ok"),
    }
}

#[test]
fn test_try_reserve_no_initial_capacity() {
    use hashbrown::HashMap;

    let mut map: HashMap<u32, String> = HashMap::new();
    assert_eq!(map.capacity(), 0);

    map.try_reserve(5).expect("Failed to reserve capacity");
    assert!(map.capacity() >= 5);
}

#[test]
fn test_try_reserve_edge_case() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, i32> = HashMap::new();

    // Trying to reserve 0 should always succeed
    map.try_reserve(0).expect("Failed to reserve zero capacity");

    // Still should have the capacity of at least 0
    assert!(map.capacity() >= 0);
}

#[test]
#[should_panic]
fn test_try_reserve_large_allocation() {
    use hashbrown::HashMap;
    use hashbrown::TryReserveError;

    let mut map: HashMap<u8, u8> = HashMap::new();

    // Attempt a very large but valid reserve, which should panic on overflow.
    let result = map.try_reserve(usize::MAX);
    if result.is_ok() {
        panic!("Expected an error but got Ok");
    }
}

