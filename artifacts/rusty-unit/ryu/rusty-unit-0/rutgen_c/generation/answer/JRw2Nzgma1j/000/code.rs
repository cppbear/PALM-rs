// Answer 0

#[test]
fn test_pow5_factor_zero() {
    assert_eq!(pow5_factor(0), 0);
}

#[test]
fn test_pow5_factor_one() {
    assert_eq!(pow5_factor(1), 0);
}

#[test]
fn test_pow5_factor_five() {
    assert_eq!(pow5_factor(5), 1);
}

#[test]
fn test_pow5_factor_ten() {
    assert_eq!(pow5_factor(10), 1);
}

#[test]
fn test_pow5_factor_twenty_five() {
    assert_eq!(pow5_factor(25), 2);
}

#[test]
fn test_pow5_factor_large() {
    assert_eq!(pow5_factor(18446744073709551615), 2);
}

#[test]
fn test_pow5_factor_boundary() {
    assert_eq!(pow5_factor(3689348814741910322), 3);
}

