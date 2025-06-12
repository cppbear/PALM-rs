// Answer 0

#[test]
fn test_invalid_byte_0_0() {
    let error = DecodeError::InvalidByte(0, 0);
    let mut output = String::new();
    let _ = fmt(&error, &mut output);
}

#[test]
fn test_invalid_byte_100_255() {
    let error = DecodeError::InvalidByte(100, 255);
    let mut output = String::new();
    let _ = fmt(&error, &mut output);
}

#[test]
fn test_invalid_byte_50_128() {
    let error = DecodeError::InvalidByte(50, 128);
    let mut output = String::new();
    let _ = fmt(&error, &mut output);
}

#[test]
fn test_invalid_byte_99_1() {
    let error = DecodeError::InvalidByte(99, 1);
    let mut output = String::new();
    let _ = fmt(&error, &mut output);
}

#[test]
fn test_invalid_byte_10_10() {
    let error = DecodeError::InvalidByte(10, 10);
    let mut output = String::new();
    let _ = fmt(&error, &mut output);
}

