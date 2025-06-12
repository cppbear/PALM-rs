// Answer 0

#[test]
fn test_indent_zero_iterations() {
    use std::io;
    use std::vec::Vec;

    // Creating a writable vector
    let mut buffer = Vec::new();
    let n = 0;
    let s = b"test";

    // Calling indent function with n = 0 should not panic and should return Ok(())
    let result = indent(&mut buffer, n, s);
    assert!(result.is_ok());
    assert!(buffer.is_empty()); // buffer should be empty as n is 0
}

#[test]
fn test_indent_non_zero_iterations() {
    use std::io;
    use std::vec::Vec;

    // Creating a writable vector
    let mut buffer = Vec::new();
    let n = 1;
    let s = b"test";

    // Calling indent function with n = 1 should return Ok(()), and buffer should contain "test"
    let result = indent(&mut buffer, n, s);
    assert!(result.is_ok());
    assert_eq!(buffer.as_slice(), b"test");
}

#[test]
fn test_indent_multiple_iterations() {
    use std::io;
    use std::vec::Vec;

    // Creating a writable vector
    let mut buffer = Vec::new();
    let n = 3;
    let s = b"test";

    // Calling indent function with n = 3 should return Ok(()), and buffer should contain "testtesttest"
    let result = indent(&mut buffer, n, s);
    assert!(result.is_ok());
    assert_eq!(buffer.as_slice(), b"testtesttest");
}

