// Answer 0

#[derive(Debug)]
struct CharInput<'t>(&'t [u8]);

#[test]
fn test_new_char_input_empty() {
    let input = new(&[]);
    assert_eq!(input.0.len(), 0);
}

#[test]
fn test_new_char_input_single_char() {
    let input = new(&[b'a']);
    assert_eq!(input.0.len(), 1);
    assert_eq!(input.0[0], b'a');
}

#[test]
fn test_new_char_input_multiple_chars() {
    let input = new(&[b'h', b'e', b'l', b'l', b'o']);
    assert_eq!(input.0.len(), 5);
    assert_eq!(input.0[0], b'h');
    assert_eq!(input.0[1], b'e');
    assert_eq!(input.0[2], b'l');
    assert_eq!(input.0[3], b'l');
    assert_eq!(input.0[4], b'o');
}

