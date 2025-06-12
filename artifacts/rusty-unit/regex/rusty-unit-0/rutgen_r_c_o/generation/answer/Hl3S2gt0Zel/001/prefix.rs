// Answer 0

#[test]
fn test_escape_byte_zero() {
    let byte = 0u8;
    escape_byte(byte);
}

#[test]
fn test_escape_byte_one() {
    let byte = 1u8;
    escape_byte(byte);
}

#[test]
fn test_escape_byte_two_fifty_five() {
    let byte = 255u8;
    escape_byte(byte);
}

#[test]
fn test_escape_byte_one_hundred() {
    let byte = 100u8;
    escape_byte(byte);
}

#[test]
fn test_escape_byte_one_twenty_seven() {
    let byte = 27u8;
    escape_byte(byte);
}

#[test]
fn test_escape_byte_two_hundred() {
    let byte = 200u8;
    escape_byte(byte);
}

