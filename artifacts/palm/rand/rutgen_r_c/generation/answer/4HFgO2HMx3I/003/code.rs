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
    let expected = (1, 1);
    assert_eq!(calculate_bound_u32(m), expected);
}

#[test]
fn test_calculate_bound_u32_two() {
    let m: u32 = 2;
    let expected = (2, 1);
    assert_eq!(calculate_bound_u32(m), expected);
}

#[test]
fn test_calculate_bound_u32_three() {
    let m: u32 = 3;
    let expected = (6, 1);
    assert_eq!(calculate_bound_u32(m), expected);
}

#[test]
fn test_calculate_bound_u32_four() {
    let m: u32 = 4;
    let expected = (24, 1);
    assert_eq!(calculate_bound_u32(m), expected);
}

#[test]
fn test_calculate_bound_u32_five() {
    let m: u32 = 5;
    let expected = (120, 1);
    assert_eq!(calculate_bound_u32(m), expected);
}

#[test]
fn test_calculate_bound_u32_six() {
    let m: u32 = 6;
    let expected = (720, 1);
    assert_eq!(calculate_bound_u32(m), expected);
}

#[test]
fn test_calculate_bound_u32_seven() {
    let m: u32 = 7;
    let expected = (5040, 1);
    assert_eq!(calculate_bound_u32(m), expected);
}

#[test]
fn test_calculate_bound_u32_eight() {
    let m: u32 = 8;
    let expected = (40320, 1);
    assert_eq!(calculate_bound_u32(m), expected);
}

#[test]
fn test_calculate_bound_u32_nine() {
    let m: u32 = 9;
    let expected = (362880, 1);
    assert_eq!(calculate_bound_u32(m), expected);
}

#[test]
fn test_calculate_bound_u32_ten() {
    let m: u32 = 10;
    let expected = (3628800, 1);
    assert_eq!(calculate_bound_u32(m), expected);
}

