// Answer 0

#[test]
fn test_reserved_byte_0x00() {
    let error = ParseAlphabetError::ReservedByte(0x00);
    let _result = format!("{}", error);
}

#[test]
fn test_reserved_byte_0x1f() {
    let error = ParseAlphabetError::ReservedByte(0x1f);
    let _result = format!("{}", error);
}

#[test]
fn test_reserved_byte_0x3a() {
    let error = ParseAlphabetError::ReservedByte(0x3a);
    let _result = format!("{}", error);
}

#[test]
fn test_reserved_byte_0x40() {
    let error = ParseAlphabetError::ReservedByte(0x40);
    let _result = format!("{}", error);
}

#[test]
fn test_reserved_byte_0x5b() {
    let error = ParseAlphabetError::ReservedByte(0x5b);
    let _result = format!("{}", error);
}

#[test]
fn test_reserved_byte_0x60() {
    let error = ParseAlphabetError::ReservedByte(0x60);
    let _result = format!("{}", error);
}

#[test]
fn test_reserved_byte_0x7b() {
    let error = ParseAlphabetError::ReservedByte(0x7b);
    let _result = format!("{}", error);
}

#[test]
fn test_reserved_byte_0x7f() {
    let error = ParseAlphabetError::ReservedByte(0x7f);
    let _result = format!("{}", error);
}

