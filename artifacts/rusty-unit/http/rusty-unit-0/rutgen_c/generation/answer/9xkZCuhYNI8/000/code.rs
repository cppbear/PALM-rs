// Answer 0

#[test]
fn test_eq_ignore_ascii_case_equal_case_insensitive() {
    let lower: &[u8] = b"hello";
    let s: &[u8] = b"HELLO";
    assert!(eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_equal_same_case() {
    let lower: &[u8] = b"world";
    let s: &[u8] = b"world";
    assert!(eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_different_length() {
    let lower: &[u8] = b"hello";
    let s: &[u8] = b"helloo";
    assert!(!eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_different_content() {
    let lower: &[u8] = b"hello";
    let s: &[u8] = b"world";
    assert!(!eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_partial_match() {
    let lower: &[u8] = b"rust";
    let s: &[u8] = b"rustacean";
    assert!(!eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_empty() {
    let lower: &[u8] = b"";
    let s: &[u8] = b"";
    assert!(eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_special_characters() {
    let lower: &[u8] = b"!";
    let s: &[u8] = b"!";
    assert!(eq_ignore_ascii_case(lower, s));
}

