// Answer 0

#[test]
fn test_advance_panic_condition_too_large() {
    use std::io::Cursor;

    let data = vec![0; 10]; // buffer of size 10
    let mut cursor = Cursor::new(data);

    cursor.set_position(5); // set position to 5
    let cnt = 6; // trying to advance by 6 which exceeds the remaining (10 - 5 = 5)

    #[should_panic(expected = "advance out of bounds: the len is 5 but advancing by 6")]
    {
        cursor.advance(cnt);
    }
}

#[test]
fn test_advance_just_enough() {
    use std::io::Cursor;

    let data = vec![0; 10]; // buffer of size 10
    let mut cursor = Cursor::new(data);

    cursor.set_position(5); // set position to 5
    let cnt = 5; // trying to advance by 5 which is exactly the remaining (10 - 5 = 5)

    cursor.advance(cnt); // this should not panic
    assert_eq!(cursor.position(), 10); // position should now be 10
}

#[test]
fn test_advance_zero() {
    use std::io::Cursor;

    let data = vec![0; 10]; // buffer of size 10
    let mut cursor = Cursor::new(data);

    cursor.set_position(5); // set position to 5
    let cnt = 0; // trying to advance by 0

    cursor.advance(cnt); // this should not panic
    assert_eq!(cursor.position(), 5); // position should remain 5
}

