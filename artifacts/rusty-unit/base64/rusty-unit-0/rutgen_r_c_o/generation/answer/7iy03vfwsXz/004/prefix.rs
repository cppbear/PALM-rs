// Answer 0

#[test]
fn test_invalid_length_0() {
    let error = ParseAlphabetError::InvalidLength;
    let mut formatter = fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_invalid_length_63() {
    let error = ParseAlphabetError::InvalidLength;
    let mut formatter = fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_invalid_length_65() {
    let error = ParseAlphabetError::InvalidLength;
    let mut formatter = fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_invalid_length_128() {
    let error = ParseAlphabetError::InvalidLength;
    let mut formatter = fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

