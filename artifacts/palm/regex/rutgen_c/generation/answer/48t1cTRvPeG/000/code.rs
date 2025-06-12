// Answer 0

#[test]
fn test_has_empty_true() {
    let flags = StateFlags(0b00000_1_00); // Set the empty bit
    assert!(flags.has_empty());
}

#[test]
fn test_has_empty_false() {
    let flags = StateFlags(0b00000_0_00); // Do not set the empty bit
    assert!(!flags.has_empty());
}

#[test]
fn test_has_empty_boundary() {
    let flags = StateFlags(0b00000_1_00); // The boundary case with empty bit set
    assert!(flags.has_empty());
    
    let flags_no_empty = StateFlags(0b00000_0_00); // The boundary case with empty bit not set
    assert!(!flags_no_empty.has_empty());
}

