// Answer 0

#[test]
fn test_capacity_to_buckets_cap_less_than_4() {
    let cap = 3; // cap < 4
    let expected = Some(4); // Expected output since cap < 4 is true
    let result = capacity_to_buckets(cap);
    assert_eq!(result, expected);
}

#[test]
fn test_capacity_to_buckets_cap_4() {
    let cap = 4; // cap == 4
    let expected = Some(8); // Expected output since cap is not less than 4
    let result = capacity_to_buckets(cap);
    assert_eq!(result, expected);
}

#[test]
fn test_capacity_to_buckets_cap_1() {
    let cap = 1; // cap < 4
    let expected = Some(4); // Expected output since cap < 4 is true
    let result = capacity_to_buckets(cap);
    assert_eq!(result, expected);
}

#[test]
fn test_capacity_to_buckets_cap_2() {
    let cap = 2; // cap < 4
    let expected = Some(4); // Expected output since cap < 4 is true
    let result = capacity_to_buckets(cap);
    assert_eq!(result, expected);
}

#[test]
fn test_capacity_to_buckets_cap_3() {
    let cap = 3; // cap < 4
    let expected = Some(4); // Expected output since cap < 4 is true
    let result = capacity_to_buckets(cap);
    assert_eq!(result, expected);
}

