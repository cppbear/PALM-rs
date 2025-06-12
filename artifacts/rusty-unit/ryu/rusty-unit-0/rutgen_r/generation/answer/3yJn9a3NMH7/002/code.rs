// Answer 0

#[test]
fn test_log2_pow5_zero() {
    assert_eq!(log2_pow5(0), 0);
}

#[test]
#[should_panic]
fn test_log2_pow5_exceed_upper_bound() {
    log2_pow5(3529);
}

