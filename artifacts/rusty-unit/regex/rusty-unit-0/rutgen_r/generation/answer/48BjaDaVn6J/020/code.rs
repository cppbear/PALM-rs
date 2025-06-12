// Answer 0

#[test]
fn test_find_cap_ref_valid_named_capture() {
    struct CaptureRef {
        cap: Ref,
        end: usize,
    }
    
    enum Ref {
        Number(usize),
        Named(&'static str),
    }

    fn is_valid_cap_letter(b: &u8) -> bool {
        (b.is_ascii_alphanumeric() || *b == b'_')
    }

    let replacement = b"${capture_name}";
    let result = find_cap_ref(replacement);
    assert!(result.is_some());
    if let Some(ref capture) = result {
        match &capture.cap {
            Ref::Named(name) => assert_eq!(name, "capture_name"),
            _ => panic!("Expected named capture"),
        }
        assert_eq!(capture.end, 17);
    }
}

#[test]
fn test_find_cap_ref_valid_number_capture() {
    struct CaptureRef {
        cap: Ref,
        end: usize,
    }
    
    enum Ref {
        Number(usize),
        Named(&'static str),
    }

    fn is_valid_cap_letter(b: &u8) -> bool {
        (b.is_ascii_alphanumeric() || *b == b'_')
    }

    let replacement = b"$123";
    let result = find_cap_ref(replacement);
    assert!(result.is_some());
    if let Some(ref capture) = result {
        match &capture.cap {
            Ref::Number(num) => assert_eq!(*num, 123),
            _ => panic!("Expected number capture"),
        }
        assert_eq!(capture.end, 4);
    }
}

#[test]
fn test_find_cap_ref_invalid_reference_no_dollar() {
    let replacement = b"capture_name";
    let result = find_cap_ref(replacement);
    assert!(result.is_none());
}

#[test]
fn test_find_cap_ref_invalid_reference_empty() {
    let replacement = b"$";
    let result = find_cap_ref(replacement);
    assert!(result.is_none());
}

#[test]
fn test_find_cap_ref_invalid_reference_no_brace() {
    let replacement = b"$capture_name";
    let result = find_cap_ref(replacement);
    assert!(result.is_some());
    if let Some(ref capture) = result {
        match &capture.cap {
            Ref::Named(name) => assert_eq!(name, "capture_name"),
            _ => panic!("Expected named capture"),
        }
        assert_eq!(capture.end, 16);
    }
}

#[test]
fn test_find_cap_ref_invalid_name_no_valid_characters() {
    let replacement = b"${}";
    let result = find_cap_ref(replacement);
    assert!(result.is_none());
}

