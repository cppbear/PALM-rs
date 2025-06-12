// Answer 0

#[test]
fn test_escape_byte_0() {
    escape_byte(0);
}

#[test]
fn test_escape_byte_1() {
    escape_byte(1);
}

#[test]
fn test_escape_byte_2() {
    escape_byte(2);
}

#[test]
fn test_escape_byte_127() {
    escape_byte(127);
}

#[test]
fn test_escape_byte_128() {
    escape_byte(128);
}

#[test]
fn test_escape_byte_255() {
    escape_byte(255);
}

#[test]
fn test_escape_byte_middle() {
    escape_byte(100);
}

#[test]
fn test_escape_byte_high_value() {
    escape_byte(200);
}

