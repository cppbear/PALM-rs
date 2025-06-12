// Answer 0

#[derive(Debug)]
struct CaptureRef {
    cap: Ref,
    end: usize,
}

#[derive(Debug)]
enum Ref {
    Number(usize),
    Named(&'static str),
}

fn is_valid_cap_letter(c: &u8) -> bool {
    (b'a'..=b'z').contains(c) || (b'A'..=b'Z').contains(c) || (b'0'..=b'9').contains(c)
}

#[test]
fn test_find_cap_ref_no_capture() {
    let input = b"$";
    let result = find_cap_ref(input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_empty_capture() {
    let input = b"${}";
    let result = find_cap_ref(input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_invalid_capture_name() {
    let input = b"${@}"; // '@' is not a valid capture letter
    let result = find_cap_ref(input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_malformed_capture_braces() {
    let input = b"${capt"; // missing closing brace
    let result = find_cap_ref(input);
    assert_eq!(result, None);
}

#[test]
fn test_find_cap_ref_valid_capture() {
    let input = b"${123}"; // "123" is a valid capture number
    let result = find_cap_ref(input);
    assert!(result.is_some());
    if let Some(cap_ref) = result {
        if let Ref::Number(num) = cap_ref.cap {
            assert_eq!(num, 123);
        } else {
            panic!("Expected a numeric capture reference");
        }
        assert_eq!(cap_ref.end, 5);
    }
}

