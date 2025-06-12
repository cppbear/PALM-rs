// Answer 0

#[test]
fn test_try_reserve_empty_map() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, isize> = HashMap::new();
    assert_eq!(map.capacity(), 0);

    map.try_reserve(10).expect("Failed to reserve space for 10 elements");
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
        _ => panic!("Expected an error due to capacity overflow"),
    }
}

