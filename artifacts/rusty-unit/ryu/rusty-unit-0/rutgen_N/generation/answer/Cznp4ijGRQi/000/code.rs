// Answer 0

#[test]
fn test_pow5factor_32_case1() {
    let value: u32 = 125; // 5^3
    assert_eq!(pow5factor_32(value), 3);
}

#[test]
fn test_pow5factor_32_case2() {
    let value: u32 = 0; // Should panic due to debug_assert
    #[should_panic]
    {
        let _ = pow5factor_32(value);
    }
}

#[test]
fn test_pow5factor_32_case3() {
    let value: u32 = 10; // 5^1 and 2 left over
    assert_eq!(pow5factor_32(value), 1);
}

#[test]
fn test_pow5factor_32_case4() {
    let value: u32 = 5; // 5^1
    assert_eq!(pow5factor_32(value), 1);
}

#[test]
fn test_pow5factor_32_case5() {
    let value: u32 = 1; // No factor of 5
    assert_eq!(pow5factor_32(value), 0);
}

#[test]
fn test_pow5factor_32_case6() {
    let value: u32 = 25; // 5^2
    assert_eq!(pow5factor_32(value), 2);
}

