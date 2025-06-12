// Answer 0

#[test]
fn test_decimal_length17_case1() {
    let v: u64 = 100_000_000_000_000; // v is exactly 100000000000000
    let result = decimal_length17(v);
    assert_eq!(result, 15);
}

#[test]
fn test_decimal_length17_case2() {
    let v: u64 = 99_999_999_999_999; // Just below the boundary
    let result = decimal_length17(v);
    assert_eq!(result, 14);
}

#[test]
fn test_decimal_length17_case3() {
    let v: u64 = 10_000_000_000_000; // Above the minimum but below the boundary
    let result = decimal_length17(v);
    assert_eq!(result, 14);
}

