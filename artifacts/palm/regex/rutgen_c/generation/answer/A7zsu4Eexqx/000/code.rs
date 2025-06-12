// Answer 0

#[test]
fn test_is_valid_cap_letter_valid_cases() {
    let valid_bytes: Vec<u8> = vec![b'0', b'5', b'9', b'a', b'g', b'z', b'A', b'F', b'Z', b'_'];
    for &byte in &valid_bytes {
        assert!(is_valid_cap_letter(&byte));
    }
}

#[test]
fn test_is_valid_cap_letter_invalid_cases() {
    let invalid_bytes: Vec<u8> = vec![b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')', b' '];
    for &byte in &invalid_bytes {
        assert!(!is_valid_cap_letter(&byte));
    }
}

#[test]
fn test_is_valid_cap_letter_edge_cases() {
    assert!(is_valid_cap_letter(&b'0'));
    assert!(is_valid_cap_letter(&b'A'));
    assert!(is_valid_cap_letter(&b'Z'));
    assert!(is_valid_cap_letter(&b'z'));
    assert!(is_valid_cap_letter(&b'_'));
    assert!(!is_valid_cap_letter(&b':'));
    assert!(!is_valid_cap_letter(&b'@'));
    assert!(!is_valid_cap_letter(&b'['));
    assert!(!is_valid_cap_letter(&b']'));
}

