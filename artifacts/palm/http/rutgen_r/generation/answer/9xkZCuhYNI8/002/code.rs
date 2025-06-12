// Answer 0

#[test]
fn test_eq_ignore_ascii_case_equal() {
    let lower: &[u8] = b"header";
    let s: &[u8] = b"HEADER";
    assert_eq!(eq_ignore_ascii_case(lower, s), true);
}

#[test]
fn test_eq_ignore_ascii_case_not_equal() {
    let lower: &[u8] = b"header";
    let s: &[u8] = b"HEAdEr";
    assert_eq!(eq_ignore_ascii_case(lower, s), false);
}

#[test]
fn test_eq_ignore_ascii_case_different_lengths() {
    let lower: &[u8] = b"header";
    let s: &[u8] = b"headerx"; // s is longer
    assert_eq!(eq_ignore_ascii_case(lower, s), false);

    let lower: &[u8] = b"headerx"; // lower is longer
    let s: &[u8] = b"header";
    assert_eq!(eq_ignore_ascii_case(lower, s), false);
}

#[test]
fn test_eq_ignore_ascii_case_empty_strings() {
    let lower: &[u8] = b"";
    let s: &[u8] = b"";
    assert_eq!(eq_ignore_ascii_case(lower, s), true);
}

#[test]
fn test_eq_ignore_ascii_case_special_characters() {
    let lower: &[u8] = b"!@#";
    let s: &[u8] = b"!@#";
    assert_eq!(eq_ignore_ascii_case(lower, s), true);
}

