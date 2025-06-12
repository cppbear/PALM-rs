// Answer 0

#[test]
fn test_pow5factor_32_case_1() {
    let value: u32 = 125; // 5^3
    let result = pow5factor_32(value);
    assert_eq!(result, 3);
}

#[test]
fn test_pow5factor_32_case_2() {
    let value: u32 = 250; // 5^3 * 2
    let result = pow5factor_32(value);
    assert_eq!(result, 3);
}

#[test]
fn test_pow5factor_32_case_3() {
    let value: u32 = 10; // 5^1 * 2
    let result = pow5factor_32(value);
    assert_eq!(result, 1);
}

#[test]
fn test_pow5factor_32_case_4() {
    let value: u32 = 0; // Edge case handling to potentially panic
    #[should_panic]
    {
        let _ = pow5factor_32(value);
    }
}

#[test]
fn test_pow5factor_32_case_5() {
    let value: u32 = 625; // 5^4
    let result = pow5factor_32(value);
    assert_eq!(result, 4);
}

