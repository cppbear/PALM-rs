// Answer 0

#[test]
fn test_find_cap_ref_no_capture() {
    let input: &[u8] = b"$";
    let result = find_cap_ref(input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_empty_capture() {
    let input: &[u8] = b"${}";
    let result = find_cap_ref(input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_invalid_characters() {
    let input: &[u8] = b"${@}";
    let result = find_cap_ref(input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_brace_without_closing() {
    let input: &[u8] = b"${capt";
    let result = find_cap_ref(input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_numeric_capture() {
    let input: &[u8] = b"$123}";
    let result = find_cap_ref(input);
    assert_eq!(result, Some(CaptureRef {
        cap: Ref::Number(123),
        end: 5,
    }));
}

#[test]
fn test_find_cap_ref_named_capture() {
    let input: &[u8] = b"${name}";
    let result = find_cap_ref(input);
    assert_eq!(result, Some(CaptureRef {
        cap: Ref::Named("name"),
        end: 6,
    }));
}

#[test]
fn test_find_cap_ref_closing_brace_missing() {
    let input: &[u8] = b"${name";
    let result = find_cap_ref(input);
    assert_eq!(result, None);
}

