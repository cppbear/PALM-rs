// Answer 0

#[test]
fn test_sign_extend_zero_byte() {
    let result = sign_extend(0, 0);
    assert_eq!(result, 0);
}

#[test]
fn test_sign_extend_one_byte_positive() {
    let result = sign_extend(0b01111111, 1);
    assert_eq!(result, 0b00000001);
}

#[test]
fn test_sign_extend_one_byte_negative() {
    let result = sign_extend(0b10000000, 1);
    assert_eq!(result, -128);
}

#[test]
fn test_sign_extend_two_bytes_positive() {
    let result = sign_extend(0b0011111111111111, 2);
    assert_eq!(result, 0b0000000000000011);
}

#[test]
fn test_sign_extend_two_bytes_negative() {
    let result = sign_extend(0b1100000000000000, 2);
    assert_eq!(result, -16384);
}

#[test]
fn test_sign_extend_three_bytes_positive() {
    let result = sign_extend(0b000000001111111111111111, 3);
    assert_eq!(result, 0b000000000000000000000111);
}

#[test]
fn test_sign_extend_three_bytes_negative() {
    let result = sign_extend(0b111111110000000000000000, 3);
    assert_eq!(result, -2097152);
}

#[test]
fn test_sign_extend_four_bytes_positive() {
    let result = sign_extend(0b00000000000000001111111111111111, 4);
    assert_eq!(result, 0b00000000000000000000000000001111);
}

#[test]
fn test_sign_extend_four_bytes_negative() {
    let result = sign_extend(0b11111111000000000000000000000000, 4);
    assert_eq!(result, -268435456);
}

#[test]
fn test_sign_extend_boundary_case_positive() {
    let result = sign_extend(0xFFFFFFFFFFFFFFFF, 8);
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF);
}

#[should_panic]
fn test_sign_extend_invalid_byte_count_too_large() {
    sign_extend(1, 9);
}

#[should_panic]
fn test_sign_extend_invalid_byte_count_zero() {
    sign_extend(1, 0);
}

