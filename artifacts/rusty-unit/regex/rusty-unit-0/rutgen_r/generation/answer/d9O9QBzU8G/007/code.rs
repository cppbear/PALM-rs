// Answer 0

#[test]
fn test_is_hex_with_constraints() {
    let c = 'a'; // Constraint: '0' <= c is false, 'a' <= c is true, c <= 'f' is true
    assert!(is_hex(c));  // Expected return value: true

    let c = 'f'; // Constraint: '0' <= c is false, 'a' <= c is true, c <= 'f' is true
    assert!(is_hex(c));  // Expected return value: true

    let c = 'g'; // Constraint: '0' <= c is false, 'a' <= c is false
    assert!(!is_hex(c)); // Expected return value: false
}

