// Answer 0

#[test]
fn test_pow5factor_32_with_non_zero_value() {
    assert_eq!(pow5factor_32(5), 1);
}

#[test]
fn test_pow5factor_32_with_larger_non_zero_value() {
    assert_eq!(pow5factor_32(25), 2);
}

#[test]
fn test_pow5factor_32_with_value_not_divisible_by_5() {
    assert_eq!(pow5factor_32(6), 0);
}

#[test]
fn test_pow5factor_32_with_value_not_multple_of_5() {
    assert_eq!(pow5factor_32(7), 0);
}

#[test]
fn test_pow5factor_32_with_higher_power_of_5() {
    assert_eq!(pow5factor_32(125), 3);
}

#[test]
fn test_pow5factor_32_with_value_just_above_a_multiple_of_5() {
    assert_eq!(pow5factor_32(15), 1);
}

