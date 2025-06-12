// Answer 0

#[test]
fn test_invalid_length_zero() {
    let error = DecodeError::InvalidLength(0);
    let _ = format!("{}", error);
}

#[test]
fn test_invalid_length_one() {
    let error = DecodeError::InvalidLength(1);
    let _ = format!("{}", error);
}

#[test]
fn test_invalid_length_five() {
    let error = DecodeError::InvalidLength(5);
    let _ = format!("{}", error);
}

