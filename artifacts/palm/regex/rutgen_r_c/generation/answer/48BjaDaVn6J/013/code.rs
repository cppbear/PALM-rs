// Answer 0

#[test]
fn test_find_cap_ref_with_named_capture() {
    let input = b"$name}";
    let result = find_cap_ref(&input);
    assert_eq!(
        result,
        Some(CaptureRef {
            cap: Ref::Named("name"),
            end: 6
        })
    );
}

#[test]
fn test_find_cap_ref_with_number_capture() {
    let input = b"$123}";
    let result = find_cap_ref(&input);
    assert_eq!(
        result,
        Some(CaptureRef {
            cap: Ref::Number(123),
            end: 5
        })
    );
}

#[test]
fn test_find_cap_ref_with_brace_open() {
    let input = b"${name}";
    let result = find_cap_ref(&input);
    assert_eq!(
        result,
        Some(CaptureRef {
            cap: Ref::Named("name"),
            end: 7
        })
    );
}

#[test]
fn test_find_cap_ref_with_missing_closing_brace() {
    let input = b"${name";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_invalid_prefix() {
    let input = b"name}";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_with_empty_string() {
    let input = b"";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_single_dollar() {
    let input = b"$";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_invalid_capture_name() {
    let input = b"${123abc}";
    let result = find_cap_ref(&input);
    assert_eq!(
        result,
        Some(CaptureRef {
            cap: Ref::Named("123abc"),
            end: 8
        })
    );
}

