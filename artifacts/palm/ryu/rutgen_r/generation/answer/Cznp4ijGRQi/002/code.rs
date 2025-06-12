// Answer 0

#[test]
#[should_panic]
fn test_pow5factor_32_zero() {
    pow5factor_32(0);
}

#[test]
fn test_pow5factor_32_one() {
    assert_eq!(pow5factor_32(1), 0);
}

#[test]
fn test_pow5factor_32_five() {
    assert_eq!(pow5factor_32(5), 1);
}

#[test]
fn test_pow5factor_32_twenty_five() {
    assert_eq!(pow5factor_32(25), 2);
}

#[test]
fn test_pow5factor_32_one_hundred() {
    assert_eq!(pow5factor_32(100), 2);
}

#[test]
fn test_pow5factor_32_one_thousand() {
    assert_eq!(pow5factor_32(1000), 3);
}

#[test]
fn test_pow5factor_32_large_value() {
    assert_eq!(pow5factor_32(3125), 5);
}

#[test]
fn test_pow5factor_32_large_non_multiple_of_five() {
    assert_eq!(pow5factor_32(124), 0);
}

