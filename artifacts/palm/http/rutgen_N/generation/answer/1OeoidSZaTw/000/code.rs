// Answer 0

#[test]
fn test_is_valid_valid_ascii() {
    assert!(is_valid(65)); // 'A'
    assert!(is_valid(32)); // space
    assert!(is_valid(33)); // '!'
}

#[test]
fn test_is_valid_invalid_ascii() {
    assert!(!is_valid(127)); // delete
    assert!(!is_valid(0));   // null character
}

#[test]
fn test_is_valid_tab() {
    assert!(is_valid(b'\t')); // tab
}

