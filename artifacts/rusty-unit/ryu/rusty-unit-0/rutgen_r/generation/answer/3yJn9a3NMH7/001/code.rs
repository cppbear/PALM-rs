// Answer 0

#[test]
fn test_log2_pow5_zero() {
    let result = log2_pow5(0);
    assert_eq!(result, 0);
}

#[test]
fn test_log2_pow5_max() {
    let result = log2_pow5(3528);
    assert_eq!(result, 1048575); // Expected output for e = 3528
}

#[test]
#[should_panic(expected = "assertion failed: e >= 0")]
fn test_log2_pow5_negative() {
    log2_pow5(-1);
}

#[test]
#[should_panic(expected = "assertion failed: e <= 3528")]
fn test_log2_pow5_above_max() {
    log2_pow5(3529);
}

