// Answer 0

#[test]
fn test_try_reserve_exact_success() {
    let mut index_map = indexmap::IndexMap::new();
    let result = index_map.try_reserve_exact(10);
    assert!(result.is_ok());
    assert_eq!(index_map.len(), 0);
}

#[test]
fn test_try_reserve_exact_zero_additional() {
    let mut index_map = indexmap::IndexMap::new();
    let result = index_map.try_reserve_exact(0);
    assert!(result.is_ok());
    assert_eq!(index_map.len(), 0);
}

#[test]
fn test_try_reserve_exact_large_additional() {
    let mut index_map = indexmap::IndexMap::new();
    let result = index_map.try_reserve_exact(1_000_000);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_try_reserve_exact_overflow() {
    let mut index_map = indexmap::IndexMap::new();
    index_map.try_reserve_exact(usize::MAX); // May panic due to overflow in capacity reservation
}

