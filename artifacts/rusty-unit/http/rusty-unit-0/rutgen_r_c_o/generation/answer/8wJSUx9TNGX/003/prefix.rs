// Answer 0

#[test]
fn test_write_checked_valid_input() {
    let src: &[u8] = b"GET";
    let mut dst = [0u8; 100];
    let _ = write_checked(src, &mut dst);
}

#[test]
fn test_write_checked_valid_input_long() {
    let src: &[u8] = b"POSTPUTDELETE";
    let mut dst = [0u8; 100];
    let _ = write_checked(src, &mut dst);
}

#[test]
fn test_write_checked_empty_input() {
    let src: &[u8] = b"";
    let mut dst = [0u8; 100];
    let _ = write_checked(src, &mut dst);
}

#[test]
fn test_write_checked_numbers() {
    let src: &[u8] = b"1234567890";
    let mut dst = [0u8; 100];
    let _ = write_checked(src, &mut dst);
}

#[test]
fn test_write_checked_special_chars() {
    let src: &[u8] = b"!@#$%^&*()";
    let mut dst = [0u8; 100];
    let _ = write_checked(src, &mut dst);
}

#[test]
fn test_write_checked_uppercase() {
    let src: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut dst = [0u8; 100];
    let _ = write_checked(src, &mut dst);
}

#[test]
fn test_write_checked_lowercase() {
    let src: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let mut dst = [0u8; 100];
    let _ = write_checked(src, &mut dst);
}

