// Answer 0

#[test]
fn test_find_cap_ref_valid_named_capture() {
    let input = b"${name}";
    let result = find_cap_ref(&input);
    assert_eq!(result, Some(CaptureRef {
        cap: Ref::Named("name"),
        end: 7,
    }));
}

#[test]
fn test_find_cap_ref_valid_number_capture() {
    let input = b"$5";
    let result = find_cap_ref(&input);
    assert_eq!(result, Some(CaptureRef {
        cap: Ref::Number(5),
        end: 2,
    }));
}

#[test]
fn test_find_cap_ref_with_brace_valid_named_capture() {
    let input = b"${myCapture}";
    let result = find_cap_ref(&input);
    assert_eq!(result, Some(CaptureRef {
        cap: Ref::Named("myCapture"),
        end: 11,
    }));
}

#[test]
fn test_find_cap_ref_invalid_capture_no_brace() {
    let input = b"$name";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_invalid_capture_empty() {
    let input = b"$";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_invalid_capture_no_valid_letter() {
    let input = b"${}";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

#[test]
#[should_panic]
fn test_find_cap_ref_panic_on_invalid_utf8() {
    let input = &[b'$', b'{', 255]; // Invalid UTF-8 sequence
    let _ = find_cap_ref(&input);
}

#[test]
fn test_find_cap_ref_non_capture() {
    let input = b"Just a string.";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

