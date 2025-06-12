// Answer 0

#[test]
fn test_at_valid_utf8_characters() {
    let input = CharInput(b"hello world");
    let result = input.at(0);
    assert_eq!(result.pos, 0);
    assert_eq!(result.c.0, 'h' as u32);
    assert_eq!(result.len, 1);
    
    let result = input.at(1);
    assert_eq!(result.pos, 1);
    assert_eq!(result.c.0, 'e' as u32);
    assert_eq!(result.len, 1);

    let result = input.at(4);
    assert_eq!(result.pos, 4);
    assert_eq!(result.c.0, 'o' as u32);
    assert_eq!(result.len, 1);
}

#[test]
#[should_panic]
fn test_at_out_of_bounds() {
    let input = CharInput(b"hello");
    let _ = input.at(10);
}

#[test]
fn test_at_valid_utf8_characters_multibyte() {
    let input = CharInput(b"\xE2\x9C\x93"); // ✓
    let result = input.at(0);
    assert_eq!(result.pos, 0);
    assert_eq!(result.c.0, 0x2713); // Unicode code point for ✓
    assert_eq!(result.len, 3);
}

#[test]
#[should_panic]
fn test_at_empty_input() {
    let input = CharInput(b"");
    let _ = input.at(0); // This should panic
}

