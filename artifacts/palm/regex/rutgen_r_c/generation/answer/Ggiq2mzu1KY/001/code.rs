// Answer 0

#[test]
fn test_is_match_true() {
    let flags = StateFlags(0b00000001);
    assert!(flags.is_match());
}

#[test]
fn test_is_match_false() {
    let flags = StateFlags(0b00000000);
    assert!(!flags.is_match());
}

#[test]
fn test_is_match_with_multiple_flags() {
    let flags = StateFlags(0b00000011);
    assert!(flags.is_match());
}

#[test]
fn test_is_match_with_high_bits() {
    let flags = StateFlags(0b10000000);
    assert!(!flags.is_match());
}

#[test]
fn test_is_match_with_zero_flags() {
    let flags = StateFlags(0);
    assert!(!flags.is_match());
}

