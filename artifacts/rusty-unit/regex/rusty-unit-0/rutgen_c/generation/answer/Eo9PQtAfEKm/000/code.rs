// Answer 0

#[test]
fn test_is_word_true() {
    let flags = StateFlags(0b000000_1_0);
    assert!(flags.is_word());
}

#[test]
fn test_is_word_false() {
    let flags = StateFlags(0b000000_0_0);
    assert!(!flags.is_word());
}

#[test]
fn test_is_word_boundary_case() {
    let flags = StateFlags(0b000000_0_1);
    assert!(!flags.is_word());
}

#[test]
fn test_is_word_high_value() {
    let flags = StateFlags(0b000000_1_1);
    assert!(flags.is_word());
}

