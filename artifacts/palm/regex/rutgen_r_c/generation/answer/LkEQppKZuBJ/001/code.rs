// Answer 0

#[test]
fn test_char_input_len_empty() {
    let input = CharInput(&[]);
    assert_eq!(input.len(), 0);
}

#[test]
fn test_char_input_len_non_empty() {
    let input = CharInput(&[b'h', b'e', b'l', b'l', b'o']);
    assert_eq!(input.len(), 5);
}

#[test]
fn test_char_input_len_single_element() {
    let input = CharInput(&[b'a']);
    assert_eq!(input.len(), 1);
}

#[test]
fn test_char_input_len_multiple_elements() {
    let input = CharInput(&[b'R', b'u', b's', b't']);
    assert_eq!(input.len(), 4);
}

#[test]
fn test_char_input_len_large_input() {
    let input = CharInput(&[b'x'; 1000]); // 1000 elements of b'x'
    assert_eq!(input.len(), 1000);
}

