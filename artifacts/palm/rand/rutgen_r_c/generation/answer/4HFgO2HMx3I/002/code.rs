// Answer 0

#[test]
fn test_calculate_bound_u32_m_equals_1() {
    let (bound, count) = calculate_bound_u32(1);
    assert_eq!(bound, 1);
    assert_eq!(count, 0);
}

#[test]
fn test_calculate_bound_u32_m_equals_2() {
    let (bound, count) = calculate_bound_u32(2);
    assert_eq!(bound, 2);
    assert_eq!(count, 1);
}

#[test]
fn test_calculate_bound_u32_m_equals_3() {
    let (bound, count) = calculate_bound_u32(3);
    assert_eq!(bound, 6);
    assert_eq!(count, 1);
}

#[test]
fn test_calculate_bound_u32_m_equals_4() {
    let (bound, count) = calculate_bound_u32(4);
    assert_eq!(bound, 24);
    assert_eq!(count, 1);
}

#[test]
fn test_calculate_bound_u32_m_equals_5() {
    let (bound, count) = calculate_bound_u32(5);
    assert_eq!(bound, 120);
    assert_eq!(count, 1);
}

#[test]
fn test_calculate_bound_u32_m_equals_12() {
    let (bound, count) = calculate_bound_u32(12);
    assert_eq!(bound, 479001600);
    assert_eq!(count, 1);
}

#[test]
#[should_panic]
fn test_calculate_bound_u32_m_equals_0() {
    let _ = calculate_bound_u32(0);
}

