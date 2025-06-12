// Answer 0

#[derive(Debug)]
struct CaptureRef {
    cap: Ref,
    end: usize,
}

#[derive(Debug)]
enum Ref {
    Number(usize),
    Named(String),
}

fn is_valid_cap_letter(b: &u8) -> bool {
    (*b as char).is_alphanumeric()
}

#[test]
fn test_find_cap_ref_valid_named_capture() {
    let input = b"${name}";
    let result = find_cap_ref(&input);
    assert!(result.is_some());
    if let Some(ref capture) = result {
        match &capture.cap {
            Ref::Named(name) => assert_eq!(name, "name"),
            _ => panic!("Expected named reference"),
        }
        assert_eq!(capture.end, 6);
    } else {
        panic!("Expected Some(CaptureRef)");
    }
}

#[test]
fn test_find_cap_ref_only_dollar_sign() {
    let input = b"$";
    let result = find_cap_ref(&input);
    assert!(result.is_none());
}

#[test]
fn test_find_cap_ref_invalid_capture_no_brace() {
    let input = b"$1"; // Valid capture reference, but without a brace around it
    let result = find_cap_ref(&input);
    assert!(result.is_some());
    if let Some(ref capture) = result {
        match &capture.cap {
            Ref::Number(num) => assert_eq!(*num, 1),
            _ => panic!("Expected number reference"),
        }
        assert_eq!(capture.end, 2);
    } else {
        panic!("Expected Some(CaptureRef)");
    }
}

#[test]
fn test_find_cap_ref_empty_input() {
    let input: &[u8] = b"";
    let result = find_cap_ref(&input);
    assert!(result.is_none());
}

#[test]
fn test_find_cap_ref_invalid_capture_unmatched_brace() {
    let input = b"${name"; // Unmatched brace
    let result = find_cap_ref(&input);
    assert!(result.is_none());
}

#[test]
fn test_find_cap_ref_non_alpha_numeric_name() {
    let input = b"${name!}"; // Invalid character in name
    let result = find_cap_ref(&input);
    assert!(result.is_none());
}

#[test]
fn test_find_cap_ref_valid_numeric_capture() {
    let input = b"$123";
    let result = find_cap_ref(&input);
    assert!(result.is_some());
    if let Some(ref capture) = result {
        match &capture.cap {
            Ref::Number(num) => assert_eq!(*num, 123),
            _ => panic!("Expected number reference"),
        }
        assert_eq!(capture.end, 4);
    } else {
        panic!("Expected Some(CaptureRef)");
    }
}

#[test]
fn test_find_cap_ref_invalid_empty_capture_name() {
    let input = b"${}";
    let result = find_cap_ref(&input);
    assert!(result.is_none());
}

