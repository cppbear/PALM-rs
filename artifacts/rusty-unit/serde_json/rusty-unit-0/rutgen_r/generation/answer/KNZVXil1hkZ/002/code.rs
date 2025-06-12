// Answer 0

#[test]
fn test_push_wtf8_codepoint_below_0x80() {
    let mut scratch = Vec::new();
    let n = 0x7F; // maximum for this range

    push_wtf8_codepoint(n, &mut scratch);

    assert_eq!(scratch.len(), 1);
    assert_eq!(scratch[0], 0x7F);
}

#[test]
fn test_push_wtf8_codepoint_exactly_0x80() {
    let mut scratch = Vec::new();
    let n = 0x80; // boundary condition 

    push_wtf8_codepoint(n, &mut scratch);

    assert_eq!(scratch.len(), 2);
    assert_eq!(scratch[0], 0b1100_0000); // 0xC0
    assert_eq!(scratch[1], 0b1000_0000); // 0x80
}

#[test]
fn test_push_wtf8_codepoint_upper_boundary_0x7FF() {
    let mut scratch = Vec::new();
    let n = 0x7FF; // maximum for this range

    push_wtf8_codepoint(n, &mut scratch);

    assert_eq!(scratch.len(), 2);
    assert_eq!(scratch[0], 0b1101_1111); // 0xDF
    assert_eq!(scratch[1], 0b1011_1111); // 0xBF
}

#[test]
fn test_push_wtf8_codepoint_minimum_above_0x7F() {
    let mut scratch = Vec::new();
    let n = 0x80 + 1; // just above the range

    push_wtf8_codepoint(n, &mut scratch);

    assert_eq!(scratch.len(), 2);
    assert_eq!(scratch[0], 0b1100_0001); // 0xC1
    assert_eq!(scratch[1], 0b1000_0001); // 0x81
}

