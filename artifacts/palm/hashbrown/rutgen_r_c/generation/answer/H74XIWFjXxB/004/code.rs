// Answer 0

#[test]
fn test_capacity_to_buckets_case_8() {
    let cap = 8;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, Some(8));
}

#[test]
fn test_capacity_to_buckets_case_9() {
    let cap = 9;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, Some(16));
}

#[test]
fn test_capacity_to_buckets_case_15() {
    let cap = 15;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, Some(32));
}

#[test]
fn test_capacity_to_buckets_edge_case() {
    let cap = usize::MAX / 7 + 1;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, None);
} 

#[test]
#[should_panic]
fn test_capacity_to_buckets_zero() {
    let cap = 0;
    capacity_to_buckets(cap);
} 


