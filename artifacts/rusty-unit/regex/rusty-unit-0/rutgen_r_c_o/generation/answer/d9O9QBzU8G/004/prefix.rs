// Answer 0

#[test]
fn test_is_hex_zero() {
    is_hex('0');
}

#[test]
fn test_is_hex_nine() {
    is_hex('9');
}

#[test]
fn test_is_hex_a() {
    is_hex('a');
}

#[test]
fn test_is_hex_f() {
    is_hex('f');
}

#[test]
fn test_is_hex_A() {
    is_hex('A');
}

#[test]
fn test_is_hex_F() {
    is_hex('F');
}

#[test]
fn test_is_hex_non_hex() {
    is_hex('g');
}

#[test]
fn test_is_hex_lower_than_zero() {
    is_hex('-');
}

#[test]
fn test_is_hex_upper_than_F() {
    is_hex('G');
}

