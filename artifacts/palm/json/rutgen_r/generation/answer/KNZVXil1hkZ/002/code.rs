// Answer 0

#[test]
fn test_push_wtf8_codepoint_below_128() {
    let mut scratch: Vec<u8> = Vec::new();
    let n: u32 = 0x20; // Within ASCII range 0..=0x7F
    push_wtf8_codepoint(n, &mut scratch);
    assert_eq!(scratch, vec![0x20]);
}

#[test]
fn test_push_wtf8_codepoint_boundary_128() {
    let mut scratch: Vec<u8> = Vec::new();
    let n: u32 = 0x80; // Boundary condition where n < 0x80 is false
    push_wtf8_codepoint(n, &mut scratch);
    assert_eq!(scratch, vec![0b1100_0000, 0b1000_0000]); // UTF-8 encoding for 0x80
}

#[test]
fn test_push_wtf8_codepoint_above_128() {
    let mut scratch: Vec<u8> = Vec::new();
    let n: u32 = 0x150; // Within the range 0x80..=0x7FF
    push_wtf8_codepoint(n, &mut scratch);
    assert_eq!(scratch, vec![0b1100_0101, 0b1000_1000]); // UTF-8 encoding for 0x150
}

#[test]
fn test_push_wtf8_codepoint_above_2048() {
    let mut scratch: Vec<u8> = Vec::new();
    let n: u32 = 0x1F600; // Within the range 0x10000..=0x10FFFF
    push_wtf8_codepoint(n, &mut scratch);
    assert_eq!(scratch, vec![0b1111_1000, 0b1001_1111, 0b1001_1000, 0b1000_0000]); // UTF-8 encoding for 0x1F600
}

