// Answer 0

#[test]
fn test_try_reserve_success() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, isize> = HashMap::new();
    assert_eq!(map.capacity(), 0);

    map.try_reserve(10).expect("Failed to reserve 10 bytes");

    assert!(map.capacity() >= 10);
}

#[test]
#[should_panic(expected = "TryReserveError::CapacityOverflow")]
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

#[test]
fn test_try_reserve_zero() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::new();
    assert_eq!(map.capacity(), 0);

    map.try_reserve(0).expect("Failed to reserve 0 bytes");

    assert!(map.capacity() >= 0);
}

#[test]
fn test_try_reserve_multiple_reserves() {
    use hashbrown::HashMap;

    let mut map: HashMap<u32, u32> = HashMap::new();
    assert_eq!(map.capacity(), 0);
    
    map.try_reserve(5).expect("Failed to reserve 5 bytes");
    assert!(map.capacity() >= 5);

    map.try_reserve(10).expect("Failed to reserve 10 bytes");
    assert!(map.capacity() >= 15);
}

#[test]
fn test_try_reserve_edge_case() {
    use hashbrown::HashMap;

    let mut map: HashMap<u8, u8> = HashMap::new();
    
    assert_eq!(map.capacity(), 0);
    assert!(map.try_reserve(1).is_ok());
    assert!(map.capacity() >= 1);
    assert!(map.try_reserve(0).is_ok());
    assert!(map.try_reserve(2).is_ok());
}

