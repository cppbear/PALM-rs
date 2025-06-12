// Answer 0

#[test]
fn test_is_hex_first_non_hex_char() {
    let c = '`'; // satisfies '0' <= c is false
    assert!(!is_hex(c));
}

#[test]
fn test_is_hex_lower_bound() {
    let c = 'A'; // satisfies 'A' <= c is true and also c <= 'F'
    assert!(is_hex(c));
}

#[test]
fn test_is_hex_upper_bound() {
    let c = 'F'; // satisfies 'A' <= c is false
    assert!(is_hex(c));
}

#[test]
fn test_is_hex_after_upper_bound() {
    let c = 'G'; // satisfies 'A' <= c is false
    assert!(!is_hex(c));
}

