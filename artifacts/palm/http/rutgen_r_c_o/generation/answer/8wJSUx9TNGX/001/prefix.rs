// Answer 0

#[test]
fn test_write_checked_valid_input_1() {
    let src: &[u8] = &[1, 2, 3];
    let mut dst = [0u8; 3];
    write_checked(src, &mut dst);
}

#[test]
fn test_write_checked_valid_input_2() {
    let src: &[u8] = &[4, 5, 6];
    let mut dst = [0u8; 3];
    write_checked(src, &mut dst);
}

#[test]
fn test_write_checked_valid_input_3() {
    let src: &[u8] = &[55, 56, 57];
    let mut dst = [0u8; 3];
    write_checked(src, &mut dst);
}

#[test]
fn test_write_checked_valid_input_4() {
    let src: &[u8] = &[97, 98, 99];
    let mut dst = [0u8; 3];
    write_checked(src, &mut dst);
}

#[test]
fn test_write_checked_valid_input_5() {
    let src: &[u8] = &[120, 121, 122];
    let mut dst = [0u8; 3];
    write_checked(src, &mut dst);
}

#[test]
#[should_panic]
fn test_write_checked_invalid_input_1() {
    let src: &[u8] = &[255];
    let mut dst = [0u8; 1];
    write_checked(src, &mut dst);
}

#[test]
#[should_panic]
fn test_write_checked_invalid_input_2() {
    let src: &[u8] = &[0];
    let mut dst = [0u8; 1];
    write_checked(src, &mut dst);
}

