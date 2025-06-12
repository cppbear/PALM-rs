// Answer 0

#[test]
fn test_push_wtf8_codepoint_bound_n_80() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x80, &mut scratch);
    assert_eq!(scratch, vec![0b1100_0000, 0b1000_0000]);
}

#[test]
fn test_push_wtf8_codepoint_bound_n_800() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x800, &mut scratch);
    assert_eq!(scratch, vec![0b1110_0000, 0b1000_0000, 0b1000_0000]);
}

#[test]
fn test_push_wtf8_codepoint_bound_n_10000() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x10000, &mut scratch);
    assert_eq!(scratch, vec![0b1111_0000, 0b1000_0000, 0b1000_0000, 0b1000_0000]);
}

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_panic_n_110000() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x110000, &mut scratch);
}

#[test]
#[should_panic]
fn test_push_wtf8_codepoint_panic_n_200000() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x200000, &mut scratch);
}

