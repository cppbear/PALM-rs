// Answer 0

#[test]
fn test_is_hex_lower_bound() {
    let c = '0';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_upper_bound() {
    let c = '9';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_with_a() {
    let c = 'a';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_with_f() {
    let c = 'f';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_with_A() {
    let c = 'A';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_with_F() {
    let c = 'F';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_with_non_hex() {
    let c = 'g';
    let result = is_hex(c);
}

