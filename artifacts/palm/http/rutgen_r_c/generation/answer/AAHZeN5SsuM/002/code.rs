// Answer 0

#[test]
fn test_to_raw_capacity_normal_case() {
    let result = to_raw_capacity(6);
    assert_eq!(result, 8);
}

#[test]
fn test_to_raw_capacity_large_input() {
    let result = to_raw_capacity(3);
    assert_eq!(result, 4);
}

#[should_panic]
fn test_to_raw_capacity_overflow_case() {
    let _ = to_raw_capacity(usize::MAX);
}

