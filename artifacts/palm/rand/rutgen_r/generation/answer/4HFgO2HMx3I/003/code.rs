// Answer 0

#[test]
#[should_panic]
fn test_calculate_bound_u32_zero() {
    let m: u32 = 0;
    calculate_bound_u32(m);
}

#[test]
fn test_calculate_bound_u32_one() {
    let m: u32 = 1;
    let (bound, count) = calculate_bound_u32(m);
    assert_eq!(bound, 1);
    assert_eq!(count, 1);
}

#[test]
fn test_calculate_bound_u32_two() {
    let m: u32 = 2;
    let (bound, count) = calculate_bound_u32(m);
    assert_eq!(bound, 2);
    assert_eq!(count, 1);
}

#[test]
fn test_calculate_bound_u32_three() {
    let m: u32 = 3;
    let (bound, count) = calculate_bound_u32(m);
    assert_eq!(bound, 6);
    assert_eq!(count, 1);
}

#[test]
fn test_calculate_bound_u32_four() {
    let m: u32 = 4;
    let (bound, count) = calculate_bound_u32(m);
    assert_eq!(bound, 24);
    assert_eq!(count, 1);
}

#[test]
fn test_calculate_bound_u32_five() {
    let m: u32 = 5;
    let (bound, count) = calculate_bound_u32(m);
    assert_eq!(bound, 120);
    assert_eq!(count, 1);
}

#[test]
fn test_calculate_bound_u32_seventeen() {
    let m: u32 = 17;
    let (bound, count) = calculate_bound_u32(m);
    assert_eq!(bound, 355687428096000);
    assert_eq!(count, 13);
}

