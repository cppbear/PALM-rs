// Answer 0

#[test]
fn test_eq_ignore_ascii_case_equal_lengths_case_insensitive_match() {
    let lower: &[u8] = b"hello";
    let s: &[u8] = b"HELLO";
    eq_ignore_ascii_case(lower, s);
}

#[test]
fn test_eq_ignore_ascii_case_equal_lengths_case_sensitive_match() {
    let lower: &[u8] = b"world";
    let s: &[u8] = b"WORLD";
    eq_ignore_ascii_case(lower, s);
}

#[test]
fn test_eq_ignore_ascii_case_equal_lengths_partial_match() {
    let lower: &[u8] = b"test";
    let s: &[u8] = b"tesT";
    eq_ignore_ascii_case(lower, s);
}

#[test]
fn test_eq_ignore_ascii_case_equal_lengths_non_match() {
    let lower: &[u8] = b"sample";
    let s: &[u8] = b"SAMPLES";
    eq_ignore_ascii_case(lower, s);
}

#[test]
fn test_eq_ignore_ascii_case_minimum_length() {
    let lower: &[u8] = b"a";
    let s: &[u8] = b"A";
    eq_ignore_ascii_case(lower, s);
}

#[test]
fn test_eq_ignore_ascii_case_maximum_length() {
    let lower: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let s: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    eq_ignore_ascii_case(lower, s);
}

#[test]
fn test_eq_ignore_ascii_case_non_ascii_input() {
    let lower: &[u8] = b"abc";
    let s: &[u8] = b"abc";
    eq_ignore_ascii_case(lower, s);
}

#[test]
fn test_eq_ignore_ascii_case_empty_case() {
    let lower: &[u8] = b"";
    let s: &[u8] = b"";
    eq_ignore_ascii_case(lower, s);
}

