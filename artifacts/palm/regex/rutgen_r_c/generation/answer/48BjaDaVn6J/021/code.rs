// Answer 0

#[test]
fn test_find_cap_ref_valid_input_no_capture() {
    let input = b"$";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_invalid_dollar() {
    let input = b"lorem ipsum";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_empty_capture() {
    let input = b"${}";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_no_valid_capture_name() {
    let input = b"${@}";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_only_dollar() {
    let input = b"$123";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

