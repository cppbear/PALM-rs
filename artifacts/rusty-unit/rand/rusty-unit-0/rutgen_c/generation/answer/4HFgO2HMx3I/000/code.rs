// Answer 0

#[test]
fn test_calculate_bound_u32_m1() {
    let result = calculate_bound_u32(1);
    assert_eq!(result, (1, 0));
}

#[test]
fn test_calculate_bound_u32_m2() {
    let result = calculate_bound_u32(2);
    assert_eq!(result, (2, 1));
}

#[test]
fn test_calculate_bound_u32_m3() {
    let result = calculate_bound_u32(3);
    assert_eq!(result, (6, 1)); // 3 * 4
}

#[test]
fn test_calculate_bound_u32_m4() {
    let result = calculate_bound_u32(4);
    assert_eq!(result, (24, 2)); // 4 * 5 * 6
}

#[test]
fn test_calculate_bound_u32_m5() {
    let result = calculate_bound_u32(5);
    assert_eq!(result, (120, 3)); // 5 * 6 * 7 * 8
}

#[test]
fn test_calculate_bound_u32_m6() {
    let result = calculate_bound_u32(6);
    assert_eq!(result, (720, 4)); // 6 * 7 * 8 * 9 * 10
}

#[test]
fn test_calculate_bound_u32_m7() {
    let result = calculate_bound_u32(7);
    assert_eq!(result, (5040, 5)); // 7 * 8 * 9 * 10 * 11 * 12
}

#[test]
fn test_calculate_bound_u32_m12() {
    let result = calculate_bound_u32(12);
    assert_eq!(result, (479001600, 13)); // 12 * ... * 24
}

#[should_panic]
fn test_calculate_bound_u32_m0() {
    let _ = calculate_bound_u32(0); // should panic due to debug_assert
}

