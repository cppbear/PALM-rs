// Answer 0

#[test]
fn test_capacity_to_buckets_zero() {
    // Testing with cap = 0, expecting panic due to the debug assertion.
    let cap = 0;
    let result = std::panic::catch_unwind(|| {
        capacity_to_buckets(cap)
    });
    assert!(result.is_err());
}

#[test]
fn test_capacity_to_buckets_four() {
    // Testing with cap = 4, expecting return of Some(8).
    let cap = 4;
    let expected = Some(8);
    let result = capacity_to_buckets(cap);
    assert_eq!(result, expected);
}

#[test]
fn test_capacity_to_buckets_seven() {
    // Testing with cap = 7, expecting return of Some(8).
    let cap = 7;
    let expected = Some(8);
    let result = capacity_to_buckets(cap);
    assert_eq!(result, expected);
}

#[test]
fn test_capacity_to_buckets_eight() {
    // Testing with cap = 8, expecting return of Some(16), since cap < 8 is false.
    let cap = 8;
    let expected = Some(16);
    let result = capacity_to_buckets(cap);
    assert_eq!(result, expected);
}

#[test]
fn test_capacity_to_buckets_max_overflow() {
    // Testing cap such that cap * 8 overflows usize. 
    let cap = usize::MAX / 8 + 1;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, None);
}

