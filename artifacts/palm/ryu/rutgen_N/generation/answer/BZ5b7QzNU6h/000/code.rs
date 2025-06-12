// Answer 0

#[test]
fn test_ceil_log2_pow5_negative() {
    assert_eq!(ceil_log2_pow5(-1), 0);
}

#[test]
fn test_ceil_log2_pow5_zero() {
    assert_eq!(ceil_log2_pow5(0), 1);
}

#[test]
fn test_ceil_log2_pow5_positive_power_of_5() {
    assert_eq!(ceil_log2_pow5(5), 4); // log2(5) is approximately 2.32, plus one is 3
}

#[test]
fn test_ceil_log2_pow5_large_number() {
    assert_eq!(ceil_log2_pow5(30), 6); // log2(30) is approximately 4.91, plus one is 5
}

#[test]
fn test_ceil_log2_pow5_edge_case() {
    assert_eq!(ceil_log2_pow5(31), 6); // log2(31) is approximately 4.95, plus one is 5
}

