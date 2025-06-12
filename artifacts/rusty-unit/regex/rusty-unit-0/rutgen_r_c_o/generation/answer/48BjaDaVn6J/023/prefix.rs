// Answer 0

#[test]
fn test_find_cap_ref_named_capture() {
    let input: &[u8] = b"${name}";
    let result = find_cap_ref(&input);
}

#[test]
fn test_find_cap_ref_number_capture_0() {
    let input: &[u8] = b"$0}";
    let result = find_cap_ref(&input);
}

#[test]
fn test_find_cap_ref_number_capture_1() {
    let input: &[u8] = b"$1}";
    let result = find_cap_ref(&input);
}

#[test]
fn test_find_cap_ref_number_capture_9() {
    let input: &[u8] = b"$9}";
    let result = find_cap_ref(&input);
}

#[test]
fn test_find_cap_ref_braced_number_capture() {
    let input: &[u8] = b"${123}";
    let result = find_cap_ref(&input);
}

#[test]
fn test_find_cap_ref_braced_named_capture() {
    let input: &[u8] = b"${captureName}";
    let result = find_cap_ref(&input);
}

#[test]
fn test_find_cap_ref_braced_empty_capture() {
    let input: &[u8] = b"${}";
    let result = find_cap_ref(&input);
}

#[test]
fn test_find_cap_ref_not_braced() {
    let input: &[u8] = b"$capture";
    let result = find_cap_ref(&input);
}

