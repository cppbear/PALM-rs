// Answer 0

#[test]
fn test_write_checked_valid_input() {
    const METHOD_CHARS: [u8; 256] = [ /* Assume valid mapping for demonstration, initialize as needed */ ];
    let src: &[u8] = &[1, 2, 3]; // Assuming 1, 2, 3 map to valid characters in METHOD_CHARS
    let mut dst: [u8; 3] = [0; 3];

    let result = write_checked(src, &mut dst);

    assert_eq!(result, Ok(()));
    assert_eq!(dst, [METHOD_CHARS[1 as usize], METHOD_CHARS[2 as usize], METHOD_CHARS[3 as usize]]);
}

#[test]
fn test_write_checked_empty_input() {
    const METHOD_CHARS: [u8; 256] = [ /* Assume valid mapping for demonstration, initialize as needed */ ];
    let src: &[u8] = &[];
    let mut dst: [u8; 0] = [];

    let result = write_checked(src, &mut dst);

    assert_eq!(result, Ok(()));
    assert_eq!(dst, []);
}

#[test]
fn test_write_checked_invalid_character() {
    const METHOD_CHARS: [u8; 256] = [0; 256]; // Every character is invalid
    let src: &[u8] = &[0]; // Input mapping to an invalid character
    let mut dst: [u8; 1] = [0];

    let result = write_checked(src, &mut dst);

    assert!(result.is_err());
}

#[test]
fn test_write_checked_boundary_exceeding_length() {
    const METHOD_CHARS: [u8; 256] = [ /* Assume valid mapping for demonstration, initialize as needed */ ];
    let src: &[u8] = &[1, 2, 3, 4]; // More elements than dst can hold
    let mut dst: [u8; 3] = [0; 3];

    let result = write_checked(src, &mut dst);

    assert!(result.is_err());
}

