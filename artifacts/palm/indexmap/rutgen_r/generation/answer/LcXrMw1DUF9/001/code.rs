// Answer 0

#[test]
fn test_try_reserve_exact_success() {
    use indexmap::IndexMap;
    use std::collections::TryReserveError;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    let initial_capacity = 10;
    map.try_reserve_exact(initial_capacity).unwrap();

    assert!(map.try_reserve_exact(5).is_ok());
    assert!(map.len() == 0); // Length should remain unchanged until keys are added
}

#[test]
fn test_try_reserve_exact_zero() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.try_reserve_exact(0).unwrap();

    assert!(map.len() == 0); // Length should still be zero
}

#[test]
#[should_panic]
fn test_try_reserve_exact_exceed_capacity() {
    use indexmap::IndexMap;
    use std::collections::TryReserveError;

    let mut map: IndexMap<i32, i32> = IndexMap::with_capacity(5);
    map.try_reserve_exact(10).unwrap(); // This should not panic, but assert the condition

    // Directly exceeding the initial capacity constraint by trying to reserve more
    assert!(map.try_reserve_exact(15).is_err());
}

#[test]
fn test_try_reserve_exact_edge_case() {
    use indexmap::IndexMap;
    use std::collections::TryReserveError;

    let mut map: IndexMap<i32, i32> = IndexMap::with_capacity(0);
    assert!(map.try_reserve_exact(0).is_ok()); // Should be successful

    assert!(map.try_reserve_exact(1).is_ok()); // Now reserving one should be fine
    assert!(map.len() == 0); // Length should still be zero
}

#[test]
fn test_try_reserve_exact_large_value() {
    use indexmap::IndexMap;
    use std::collections::TryReserveError;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    assert!(map.try_reserve_exact(1_000_000).is_ok()); // Test with large value
}

