// Answer 0

#[test]
fn test_eq_ignore_ascii_case_equal() {
    let lower = b"header";
    let s = b"HEADER";
    assert!(eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_different_length() {
    let lower = b"header";
    let s = b"head";
    assert!(!eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_different_content() {
    let lower = b"header";
    let s = b"HEAder";
    assert!(!eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_empty() {
    let lower = b"";
    let s = b"";
    assert!(eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_case_insensitive() {
    let lower = b"case";
    let s = b"CASe";
    assert!(eq_ignore_ascii_case(lower, s));
}

#[test]
#[should_panic]
fn test_eq_ignore_ascii_case_panic() {
    let lower = b"panic";
    let s = b"PANIC!";
    assert!(eq_ignore_ascii_case(lower, s)); // this should panic on length mismatch
}

