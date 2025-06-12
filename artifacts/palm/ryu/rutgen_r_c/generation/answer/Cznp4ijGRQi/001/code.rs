// Answer 0

#[test]
fn test_pow5factor_32_no_factor_of_five() {
    let value = 1; // r will not equal 0
    let result = pow5factor_32(value);
    assert_eq!(result, 0); // since 1 has 0 factors of 5
}

#[test]
fn test_pow5factor_32_one_factor_of_five() {
    let value = 5; // only one factor of 5
    let result = pow5factor_32(value);
    assert_eq!(result, 1); // 5 has one factor of 5
}

#[test]
fn test_pow5factor_32_two_factors_of_five() {
    let value = 25; // 5^2 has two factors of 5
    let result = pow5factor_32(value);
    assert_eq!(result, 2); // 25 has two factors of 5
}

#[test]
fn test_pow5factor_32_three_factors_of_five() {
    let value = 125; // 5^3 has three factors of 5
    let result = pow5factor_32(value);
    assert_eq!(result, 3); // 125 has three factors of 5
}

#[test]
fn test_pow5factor_32_edge_case_large_value() {
    let value = 3125; // 5^5 has five factors of 5
    let result = pow5factor_32(value);
    assert_eq!(result, 5); // 3125 has five factors of 5
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_pow5factor_32_panic_on_zero() {
    let value = 0; // this should trigger a panic
    pow5factor_32(value);
}

