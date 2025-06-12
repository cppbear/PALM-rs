// Answer 0

#[test]
fn test_at_valid_position() {
    let input = CharInput(b"hello".to_vec().as_slice());
    let result = input.at(0);
    assert_eq!(result.pos, 0);
    assert_eq!(result.c.0, 'h' as u32);
    assert_eq!(result.len, 1);
}

#[test]
fn test_at_middle_position() {
    let input = CharInput(b"hello".to_vec().as_slice());
    let result = input.at(2);
    assert_eq!(result.pos, 2);
    assert_eq!(result.c.0, 'l' as u32);
    assert_eq!(result.len, 1);
}

#[test]
fn test_at_end_position() {
    let input = CharInput(b"hello".to_vec().as_slice());
    let result = input.at(4);
    assert_eq!(result.pos, 4);
    assert_eq!(result.c.0, 'o' as u32);
    assert_eq!(result.len, 1);
}

#[test]
fn test_at_out_of_bounds() {
    let input = CharInput(b"hello".to_vec().as_slice());
    let result = input.at(5); // This case may need better handling standards in a real scenario.
    assert_eq!(result.pos, 5);
    assert!(result.c.is_none());
    assert_eq!(result.len, 0);
}

