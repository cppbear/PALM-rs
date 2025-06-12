// Answer 0

#[test]
fn test_as_byte_with_zero() {
    let byte = Byte(0);
    byte.as_byte();
}

#[test]
fn test_as_byte_with_one() {
    let byte = Byte(1);
    byte.as_byte();
}

#[test]
fn test_as_byte_with_two_fifty_four() {
    let byte = Byte(254);
    byte.as_byte();
}

#[test]
fn test_as_byte_with_two_fifty_five() {
    let byte = Byte(255);
    byte.as_byte();
}

