// Answer 0

#[test]
fn test_find_cap_ref_valid_number_reference() {
    struct CaptureRef {
        cap: Ref,
        end: usize,
    }

    enum Ref {
        Number(usize),
        Named(&'static str),
    }

    fn is_valid_cap_letter(byte: &u8) -> bool {
        *byte >= b'a' && *byte <= b'z' || *byte >= b'0' && *byte <= b'9'
    }

    let input = b"${123}";
    let result = find_cap_ref(input);
    assert_eq!(result, Some(CaptureRef {
        cap: Ref::Number(123),
        end: 5,
    }));
}

#[test]
fn test_find_cap_ref_valid_named_reference() {
    struct CaptureRef {
        cap: Ref,
        end: usize,
    }

    enum Ref {
        Number(usize),
        Named(&'static str),
    }

    fn is_valid_cap_letter(byte: &u8) -> bool {
        *byte >= b'a' && *byte <= b'z'
    }

    let input = b"${name}";
    let result = find_cap_ref(input);
    assert_eq!(result, Some(CaptureRef {
        cap: Ref::Named("name"),
        end: 6,
    }));
}

#[test]
fn test_find_cap_ref_invalid_starting_char() {
    let input = b"abc{def}";
    let result = find_cap_ref(input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_no_capture_group() {
    let input = b"$";
    let result = find_cap_ref(input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_invalid_capture_group() {
    let input = b"${}";
    let result = find_cap_ref(input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_missing_closing_brace() {
    let input = b"${name";
    let result = find_cap_ref(input);
    assert_eq!(result, None);
}

