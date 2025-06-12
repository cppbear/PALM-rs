// Answer 0

#[test]
fn test_new_empty() {
    let _ = AllocatedExtension::new(&[]);
}

#[test]
fn test_new_single_zero() {
    let result = AllocatedExtension::new(&[0]);
}

#[test]
fn test_new_single_invalid() {
    let result = AllocatedExtension::new(&[255]);
}

#[test]
fn test_new_single_high() {
    let result = AllocatedExtension::new(&[254]);
}

#[test]
fn test_new_single_null() {
    let result = AllocatedExtension::new(&[b'\0']);
}

#[test]
fn test_new_valid_mixed_with_null() {
    let _ = AllocatedExtension::new(&[b'!', b'\0', b'#']);
}

#[test]
fn test_new_valid_alphabet() {
    let _ = AllocatedExtension::new(&[b'A', b'Z', b'a', b'z']);
}

#[test]
fn test_new_valid_special_chars() {
    let _ = AllocatedExtension::new(&[b'!', b'#', b'$', b'%', b'&', b'\'', b'`']);
}

#[test]
fn test_new_valid_numeric_and_special() {
    let _ = AllocatedExtension::new(&[b'~', b'|', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9']);
}

#[test]
fn test_new_valid_mixed_string_with_special() {
    let _ = AllocatedExtension::new(&[b'A', b'E', b'G', b'U', b'z', b'c', b'q', b'|']);
}

#[test]
fn test_new_invalid_with_null_and_special() {
    let result = AllocatedExtension::new(&[b'\0', b'~', b'*', b'+', b'-', b'%', b'&', b'c', b'i', b'j']);
}

