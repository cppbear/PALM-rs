// Answer 0

#[test]
fn test_advance_valid_advance() {
    use std::io::Cursor;

    let mut cursor = Cursor::new(vec![0u8, 1, 2, 3, 4]);
    let initial_position = cursor.position();

    cursor.advance(2);
    
    assert_eq!(cursor.position(), initial_position + 2);
}

#[test]
#[should_panic(expected = "advance out of bounds")]
fn test_advance_out_of_bounds() {
    use std::io::Cursor;

    let mut cursor = Cursor::new(vec![0u8, 1]);
    cursor.advance(3);
}

#[test]
fn test_advance_zero() {
    use std::io::Cursor;

    let mut cursor = Cursor::new(vec![0u8, 1, 2, 3, 4]);
    let initial_position = cursor.position();

    cursor.advance(0);
    
    assert_eq!(cursor.position(), initial_position);
}

#[test]
fn test_advance_exceeding_but_valid() {
    use std::io::Cursor;

    let mut cursor = Cursor::new(vec![0u8, 1, 2]);
    cursor.advance(2);

    assert_eq!(cursor.position(), 2);
}

