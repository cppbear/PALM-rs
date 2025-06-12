// Answer 0

#[test]
fn test_eq_ignore_ascii_case_different_lengths() {
    let lower: &[u8] = b"hello";
    let s: &[u8] = b"helloo"; // different length, should return false
    assert!(!eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_empty_lower() {
    let lower: &[u8] = b""; // empty lower
    let s: &[u8] = b"some"; // different length, should return false
    assert!(!eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_empty_s() {
    let lower: &[u8] = b"some"; // different length, should return false
    let s: &[u8] = b""; // empty s
    assert!(!eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_different_case() {
    let lower: &[u8] = b"abc"; 
    let s: &[u8] = b"Ab"; // different length, should return false
    assert!(!eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_special_chars() {
    let lower: &[u8] = b"!@#"; 
    let s: &[u8] = b"!@#$"; // different length, should return false
    assert!(!eq_ignore_ascii_case(lower, s));
}

