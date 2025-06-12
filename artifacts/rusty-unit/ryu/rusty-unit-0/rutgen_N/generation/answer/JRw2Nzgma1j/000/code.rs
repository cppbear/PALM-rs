// Answer 0

#[test]
fn test_pow5_factor_basic() {
    assert_eq!(pow5_factor(1), 0);
    assert_eq!(pow5_factor(5), 1);
    assert_eq!(pow5_factor(25), 2);
    assert_eq!(pow5_factor(125), 3);
    assert_eq!(pow5_factor(625), 4);
}

#[test]
fn test_pow5_factor_large() {
    assert_eq!(pow5_factor(0), 0); // edge case: 0 should be handled
    assert_eq!(pow5_factor(3125), 5);
    assert_eq!(pow5_factor(15625), 6);
}

#[test]
fn test_pow5_factor_edge() {
    assert_eq!(pow5_factor(1 << 63), 0); // very large number
    assert_eq!(pow5_factor(u64::MAX), 0); // max value of u64
}

