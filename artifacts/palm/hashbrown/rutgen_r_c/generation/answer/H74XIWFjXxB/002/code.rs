// Answer 0

#[test]
fn test_capacity_to_buckets_small_case_under_4() {
    let cap = 3;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, Some(4));
}

#[test]
fn test_capacity_to_buckets_small_case_4() {
    let cap = 4;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, Some(8));
}

#[test]
fn test_capacity_to_buckets_small_case_2() {
    let cap = 2;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, Some(4));
}

#[test]
fn test_capacity_to_buckets_small_case_1() {
    let cap = 1;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, Some(4));
}

