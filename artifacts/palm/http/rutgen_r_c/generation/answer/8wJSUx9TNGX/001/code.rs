// Answer 0

#[test]
fn test_write_checked_with_invalid_method() {
    let input: &[u8] = &[0, 1, 2, 255]; // Includes 0 which is invalid
    let mut output = [0u8; 4]; // An appropriate size for the output buffer

    let result = write_checked(input, &mut output);
    assert!(result.is_err());
}

#[test]
fn test_write_checked_with_valid_characters() {
    let input: &[u8] = &[1, 2, 3, 4, 65, 66, 67]; // All valid indices
    let mut output = [0u8; 7]; // Adjust size accordingly

    let result = write_checked(input, &mut output);
    assert!(result.is_ok());
    assert_eq!(output, [b'!', b'#', b'$', b'%', b'A', b'B', b'C']); // Expected mapping
}

#[test]
fn test_write_checked_with_multiple_invalid() {
    let input: &[u8] = &[0, 0, 0]; // All values are invalid
    let mut output = [0u8; 3];

    let result = write_checked(input, &mut output);
    assert!(result.is_err());
}

#[test]
fn test_write_checked_empty_input() {
    let input: &[u8] = &[]; // Edge case: empty input
    let mut output = [0u8; 0]; // No output needed

    let result = write_checked(input, &mut output);
    assert!(result.is_ok());
}

#[test]
fn test_write_checked_boundary_index() {
    let input: &[u8] = &[254, 255]; // Test maximum valid indices
    let mut output = [0u8; 2];

    let result = write_checked(input, &mut output);
    assert!(result.is_ok());
    assert_eq!(output, [b'~', b'\0']); // Ensure that the output matches the expected mapping
}

