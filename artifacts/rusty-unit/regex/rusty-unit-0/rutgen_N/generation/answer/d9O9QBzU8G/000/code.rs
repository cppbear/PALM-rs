// Answer 0

#[test]
fn test_is_hex() {
    assert!(is_hex('0'));
    assert!(is_hex('9'));
    assert!(is_hex('a'));
    assert!(is_hex('f'));
    assert!(is_hex('A'));
    assert!(is_hex('F'));
    assert!(!is_hex('g'));
    assert!(!is_hex('Z'));
    assert!(!is_hex(' '));
    assert!(!is_hex('1'));
    assert!(!is_hex('!'));
}

