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
fn test_is_word_boundary_max_value() {
    let flags = StateFlags(u8::MAX);
    assert!(flags.is_word()); // 0b11111111 & 0b00000001 > 0
}

#[test]
fn test_is_word_boundary_zero() {
    let flags = StateFlags(0);
    assert!(!flags.is_word()); // 0b00000000 & 0b00000001 > 0 is false
}

#[test]
fn test_is_word_boundary_high_bit() {
    let flags = StateFlags(0b10000000); // highest bit set
    assert!(!flags.is_word()); // 0b10000000 & 0b00000001 > 0 is false
}

#[test]
fn test_is_word_other_high_bits() {
    let flags = StateFlags(0b11111110);
    assert!(!flags.is_word()); // high bits do not affect the last two bits
}

