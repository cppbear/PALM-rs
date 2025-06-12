// Answer 0

#[test]
fn test_find_cap_ref_valid_named_capture() {
    let input = b"${name}";
    let result = find_cap_ref(&input);
    assert_eq!(result, Some(CaptureRef {
        cap: Ref::Named("name"),
        end: 6,
    }));
}

#[test]
fn test_find_cap_ref_valid_number_capture() {
    let input = b"$123}";
    let result = find_cap_ref(&input);
    assert_eq!(result, Some(CaptureRef {
        cap: Ref::Number(123),
        end: 4,
    }));
}

#[test]
fn test_find_cap_ref_no_capture() {
    let input = b"$}";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_missing_closing_brace() {
    let input = b"${name";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_invalid_character_after_capture() {
    let input = b"${name}x";
    let result = find_cap_ref(&input);
    assert_eq!(result, Some(CaptureRef {
        cap: Ref::Named("name"),
        end: 7,
    }));
}

#[test]
fn test_find_cap_ref_empty_capture() {
    let input = b"${}";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_valid_capture_without_brace() {
    let input = b"$valid";
    let result = find_cap_ref(&input);
    assert_eq!(result, Some(CaptureRef {
        cap: Ref::Named("valid"),
        end: 6,
    }));
}

