// Answer 0

#[test]
fn test_sign_extend_zero_bytes() {
    let val = 0u64;
    let nbytes = 0;
    let result = sign_extend(val, nbytes);
    assert_eq!(result, 0);
}

#[test]
fn test_sign_extend_one_byte() {
    let val = 0xFFu64; // All bits set for one byte.
    let nbytes = 1;
    let result = sign_extend(val, nbytes);
    assert_eq!(result, -1);
}

#[test]
fn test_sign_extend_two_bytes() {
    let val = 0xFFFFu64; // All bits set for two bytes.
    let nbytes = 2;
    let result = sign_extend(val, nbytes);
    assert_eq!(result, -1);
}

#[test]
fn test_sign_extend_three_bytes() {
    let val = 0xFFFFFFu64; // All bits set for three bytes.
    let nbytes = 3;
    let result = sign_extend(val, nbytes);
    assert_eq!(result, -1);
}

#[test]
fn test_sign_extend_four_bytes() {
    let val = 0xFFFFFFFFu64; // All bits set for four bytes.
    let nbytes = 4;
    let result = sign_extend(val, nbytes);
    assert_eq!(result, -1);
}

#[test]
fn test_sign_extend_five_bytes() {
    let val = 0xFFFFFFFFFFu64; // Example value for five bytes.
    let nbytes = 5;
    let result = sign_extend(val, nbytes);
    assert_eq!(result, -1);
}

#[test]
fn test_sign_extend_six_bytes() {
    let val = 0xFFFFFFFFFFFFu64; // Example value for six bytes.
    let nbytes = 6;
    let result = sign_extend(val, nbytes);
    assert_eq!(result, -1);
}

#[test]
fn test_sign_extend_seven_bytes() {
    let val = 0xFFFFFFFFFFFFFFu64; // Example value for seven bytes.
    let nbytes = 7;
    let result = sign_extend(val, nbytes);
    assert_eq!(result, -1);
}

#[test]
fn test_sign_extend_eight_bytes() {
    let val = 0x7FFFFFFFFFFFFFFFu64; // Max positive value for signed 64-bit int.
    let nbytes = 8;
    let result = sign_extend(val, nbytes);
    assert_eq!(result, 0x7FFFFFFFFFFFFFFF);
} 

#[test]
fn test_sign_extend_eight_bytes_negative() {
    let val = 0xFFFFFFFFFFFFFFFFu64; // Max negative value for signed 64-bit int.
    let nbytes = 8;
    let result = sign_extend(val, nbytes);
    assert_eq!(result, -1);
} 

#[should_panic]
fn test_sign_extend_invalid_nbytes() {
    let val = 0u64;
    let nbytes = 9; // nbytes cannot exceed 8.
    sign_extend(val, nbytes);
}

