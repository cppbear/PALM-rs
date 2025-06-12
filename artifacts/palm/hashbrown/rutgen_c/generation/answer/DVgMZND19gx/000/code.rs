// Answer 0

#[test]
fn test_try_reserve_success() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, isize> = HashMap::new();
    map.try_reserve(10).expect("should successfully reserve 10 additional spaces");
    assert!(map.capacity() >= 10);
}

#[test]
#[should_panic(expected = "TryReserveError::CapacityOverflow")]
fn test_try_reserve_capacity_overflow() {
    use hashbrown::HashMap;
    use hashbrown::TryReserveError;

    let mut map: HashMap<i32, i32> = HashMap::new();

    match map.try_reserve(usize::MAX) {
        Err(TryReserveError::CapacityOverflow) => {}
        _ => panic!("Expected TryReserveError::CapacityOverflow"),
    }
}

