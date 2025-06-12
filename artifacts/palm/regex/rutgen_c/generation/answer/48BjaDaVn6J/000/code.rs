// Answer 0

#[test]
fn test_find_cap_ref_named_capture() {
    let input = b"${name}";
    let expected = CaptureRef {
        cap: Ref::Named("name"),
        end: 6,
    };
    let result = find_cap_ref(&input);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_find_cap_ref_numeric_capture() {
    let input = b"$1";
    let expected = CaptureRef {
        cap: Ref::Number(1),
        end: 2,
    };
    let result = find_cap_ref(&input);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_find_cap_ref_no_capture() {
    let input = b"$";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_invalid_no_closing_brace() {
    let input = b"${name";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_large_number() {
    let input = b"$123456789";
    let expected = CaptureRef {
        cap: Ref::Number(123456789),
        end: 12,
    };
    let result = find_cap_ref(&input);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_find_cap_ref_braced_capture() {
    let input = b"${123}";
    let expected = CaptureRef {
        cap: Ref::Number(123),
        end: 5,
    };
    let result = find_cap_ref(&input);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_find_cap_ref_no_valid_capture() {
    let input = b"$%";
    let result = find_cap_ref(&input);
    assert_eq!(result, None);
}

