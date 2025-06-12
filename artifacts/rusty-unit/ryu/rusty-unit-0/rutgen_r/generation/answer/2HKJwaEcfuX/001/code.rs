// Answer 0

#[test]
fn test_log10_pow2_zero() {
    let result = log10_pow2(0);
    assert_eq!(result, 0);
}

#[test]
fn test_log10_pow2_max() {
    let result = log10_pow2(1650);
    assert_eq!(result, 78847);
}

