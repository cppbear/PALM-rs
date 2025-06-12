// Answer 0

#[test]
fn test_capacity_to_buckets_with_cap_4() {
    let cap: usize = 4;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, Some(8));
}

#[test]
fn test_capacity_to_buckets_with_cap_3() {
    let cap: usize = 3;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, Some(4));
}

#[test]
fn test_capacity_to_buckets_with_cap_0() {
    let cap: usize = 0;
    let result = capacity_to_buckets(cap);
    // The function has a debug_assert_ne!(cap, 0), thus it will panic
    let result = std::panic::catch_unwind(|| capacity_to_buckets(cap));
    assert!(result.is_err());
}

