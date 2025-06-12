// Answer 0

#[test]
fn test_is_hex_with_0() {
    let c = '0';
    assert_eq!(is_hex(c), true);
}

#[test]
fn test_is_hex_with_9() {
    let c = '9';
    assert_eq!(is_hex(c), true);
}

#[test]
fn test_is_hex_with_a() {
    let c = 'a';
    assert_eq!(is_hex(c), true);
}

#[test]
fn test_is_hex_with_f() {
    let c = 'f';
    assert_eq!(is_hex(c), true);
}

#[test]
fn test_is_hex_with_A() {
    let c = 'A';
    assert_eq!(is_hex(c), false);
}

#[test]
fn test_is_hex_with_B() {
    let c = 'B';
    assert_eq!(is_hex(c), false);
}

#[test]
fn test_is_hex_with_C() {
    let c = 'C';
    assert_eq!(is_hex(c), false);
}

#[test]
fn test_is_hex_with_D() {
    let c = 'D';
    assert_eq!(is_hex(c), false);
}

#[test]
fn test_is_hex_with_E() {
    let c = 'E';
    assert_eq!(is_hex(c), false);
}

#[test]
fn test_is_hex_with_F() {
    let c = 'F';
    assert_eq!(is_hex(c), false);
}

#[test]
fn test_is_hex_with_non_hex() {
    let c = 'g';
    assert_eq!(is_hex(c), false);
}

