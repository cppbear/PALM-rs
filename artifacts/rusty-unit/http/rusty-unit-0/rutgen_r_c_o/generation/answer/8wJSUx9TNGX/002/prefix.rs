// Answer 0

#[test]
fn test_write_checked_valid_input() {
    let src: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut dst: [u8; 10] = [0; 10];
    let _ = write_checked(src, &mut dst);
}

#[test]
fn test_write_checked_valid_input_large() {
    let src: &[u8] = &[10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
    let mut dst: [u8; 10] = [0; 10];
    let _ = write_checked(src, &mut dst);
}

#[test]
fn test_write_checked_invalid_input() {
    let src: &[u8] = &[20];
    let mut dst: [u8; 1] = [0; 1];
    let _ = write_checked(src, &mut dst);
}

