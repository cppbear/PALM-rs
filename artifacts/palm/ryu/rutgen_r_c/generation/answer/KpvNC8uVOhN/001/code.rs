// Answer 0

#[test]
fn test_pow5bits_lower_bound() {
    let e: i32 = 0;
    let result = pow5bits(e);
    assert_eq!(result, 1);
}

#[test]
fn test_pow5bits_upper_bound() {
    let e: i32 = 3528;
    let result = pow5bits(e);
    assert_eq!(result, 1161266);
}

#[test]
#[should_panic]
fn test_pow5bits_negative_input() {
    let e: i32 = -1;
    let _ = pow5bits(e);
}

#[test]
#[should_panic]
fn test_pow5bits_exceeding_upper_bound() {
    let e: i32 = 3529;
    let _ = pow5bits(e);
}

