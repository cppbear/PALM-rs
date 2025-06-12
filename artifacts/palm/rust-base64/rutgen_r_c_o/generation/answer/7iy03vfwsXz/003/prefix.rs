// Answer 0

#[test]
fn test_fmt_duplicated_byte_0() {
    let error = ParseAlphabetError::DuplicatedByte(0);
    let mut buffer = String::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_fmt_duplicated_byte_1() {
    let error = ParseAlphabetError::DuplicatedByte(1);
    let mut buffer = String::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_fmt_duplicated_byte_2() {
    let error = ParseAlphabetError::DuplicatedByte(2);
    let mut buffer = String::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_fmt_duplicated_byte_254() {
    let error = ParseAlphabetError::DuplicatedByte(254);
    let mut buffer = String::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_fmt_duplicated_byte_255() {
    let error = ParseAlphabetError::DuplicatedByte(255);
    let mut buffer = String::new();
    let _ = error.fmt(&mut buffer);
}

