// Answer 0

#[test]
fn test_alphabetic_char() {
    let result = fastrand::alphabetic();
    assert!(result.is_ascii_alphabetic(), "Result should be an ASCII alphabetic character");
}

#[test]
fn test_alphabetic_char_range() {
    for _ in 0..1000 {
        let result = fastrand::alphabetic();
        assert!(result >= 'a' && result <= 'z' || result >= 'A' && result <= 'Z', "Result should be in the alphabetic range a-z or A-Z");
    }
}

#[should_panic]
fn test_alphabetic_invalid_character() {
    // This test is meant to check the panic situation (hypothetical as we trust the implementation here)
    // However, if we expect alphabetic function to only return valid characters, this shouldn't panic.
    let result = fastrand::alphabetic();
    assert!(!result.is_ascii_alphabetic(), "Result should not be valid");
}

