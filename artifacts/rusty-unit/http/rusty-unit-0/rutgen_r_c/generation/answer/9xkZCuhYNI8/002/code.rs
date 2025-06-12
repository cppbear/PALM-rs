// Answer 0

#[test]
fn test_eq_ignore_ascii_case_equal_lower_and_upper() {
    let lower = b"hello";
    let s = b"HELLO";
    assert!(eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_same_length_different_cases() {
    let lower = b"fOoBar";
    let s = b"FoObAr";
    assert!(eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_different_length() {
    let lower = b"hello";
    let s = b"helloo";  // Different length
    assert!(!eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_with_special_character() {
    let lower = b"test!";
    let s = b"TEST!";  // Special character remains unchanged
    assert!(eq_ignore_ascii_case(lower, s));
}

#[test]
fn test_eq_ignore_ascii_case_mixed_case_with_non_ascii() {
    let lower = b"teSt123";
    let s = b"TEST123";  // Numeric characters are the same
    assert!(eq_ignore_ascii_case(lower, s));
}

#[test]
#[should_panic]
fn test_eq_ignore_ascii_case_out_of_bounds() {
    let lower = b"abc";
    let s = b"\xFF"; // Causes panic due to out of bounds access
    let _ = eq_ignore_ascii_case(lower, s);
}

