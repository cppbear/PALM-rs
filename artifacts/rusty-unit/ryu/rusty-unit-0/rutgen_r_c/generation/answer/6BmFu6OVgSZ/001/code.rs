// Answer 0

#[test]
fn test_multiple_of_power_of_5_case_1() {
    let value: u64 = 125; // 5^3, should pass for p = 3
    let p: u32 = 3;
    assert_eq!(multiple_of_power_of_5(value, p), true);
}

#[test]
fn test_multiple_of_power_of_5_case_2() {
    let value: u64 = 100; // 5^2 * 4, should pass for p = 2
    let p: u32 = 2;
    assert_eq!(multiple_of_power_of_5(value, p), true);
}

#[test]
fn test_multiple_of_power_of_5_case_3() {
    let value: u64 = 24; // No factors of 5, should fail for any p > 0
    let p: u32 = 1;
    assert_eq!(multiple_of_power_of_5(value, p), false);
}

#[test]
fn test_multiple_of_power_of_5_case_4() {
    let value: u64 = 0; // Edge case: should panic since value cannot be 0 in pow5_factor
    let p: u32 = 1;
    let result = std::panic::catch_unwind(|| {
        multiple_of_power_of_5(value, p)
    });
    assert!(result.is_err());
}

#[test]
fn test_multiple_of_power_of_5_case_5() {
    let value: u64 = 3125; // 5^5, should pass for p = 5
    let p: u32 = 5;
    assert_eq!(multiple_of_power_of_5(value, p), true);
}

#[test]
fn test_multiple_of_power_of_5_case_6() {
    let value: u64 = 15625; // 5^6, should pass for p = 6
    let p: u32 = 6;
    assert_eq!(multiple_of_power_of_5(value, p), true);
}

#[test]
fn test_multiple_of_power_of_5_case_7() {
    let value: u64 = 1; // No factors of 5, should fail for p > 0
    let p: u32 = 1;
    assert_eq!(multiple_of_power_of_5(value, p), false);
}

#[test]
fn test_multiple_of_power_of_5_case_8() {
    let value: u64 = 5; // 5^1, should pass for p = 1
    let p: u32 = 1;
    assert_eq!(multiple_of_power_of_5(value, p), true);
}

#[test]
fn test_multiple_of_power_of_5_edge_case() {
    let value: u64 = u64::MAX; // Should run but with no potential for a large power of 5
    let p: u32 = 0; // Any non-negative p should return true
    assert_eq!(multiple_of_power_of_5(value, p), true);
}

