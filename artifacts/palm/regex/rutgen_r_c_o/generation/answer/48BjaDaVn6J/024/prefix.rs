// Answer 0

#[test]
fn test_find_cap_ref_valid_named_capture() {
    let input = b"$foo{123}";
    let result = find_cap_ref(&input);
}

#[test]
fn test_find_cap_ref_missing_closing_brace() {
    let input = b"$foo{123";
    let result = find_cap_ref(&input);
}

#[test]
fn test_find_cap_ref_valid_number_capture() {
    let input = b"$123{456}";
    let result = find_cap_ref(&input);
}

#[test]
fn test_find_cap_ref_empty_capture() {
    let input = b"$";
    let result = find_cap_ref(&input);
}

#[test]
fn test_find_cap_ref_no_capture_name() {
    let input = b"${}";
    let result = find_cap_ref(&input);
}

#[test]
fn test_find_cap_ref_invalid_character_after_capture() {
    let input = b"$foo{1x23}";
    let result = find_cap_ref(&input);
}

#[test]
fn test_find_cap_ref_capture_with_extra_characters() {
    let input = b"$foo{123}abc";
    let result = find_cap_ref(&input);
}

#[test]
fn test_find_cap_ref_invalid_start_character() {
    let input = b"foo{123}";
    let result = find_cap_ref(&input);
}

#[test]
fn test_find_cap_ref_only_closing_brace() {
    let input = b"$}";
    let result = find_cap_ref(&input);
}

#[test]
fn test_find_cap_ref_open_brace_without_name() {
    let input = b"$ {";
    let result = find_cap_ref(&input);
}

