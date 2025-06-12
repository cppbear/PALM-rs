// Answer 0

#[test]
fn test_capacity_to_buckets_case_1() {
    let cap = 4;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, Some(8));
}

#[test]
#[should_panic]
fn test_capacity_to_buckets_case_2() {
    let cap = 0; // This will trigger a debug assertion
    let _result = capacity_to_buckets(cap);
}

