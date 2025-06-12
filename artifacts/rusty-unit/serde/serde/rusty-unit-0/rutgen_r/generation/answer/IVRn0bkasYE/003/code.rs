// Answer 0

#[test]
fn test_format_u8_below_ten() {
    let mut output = [0u8; 3]; // Output buffer
    let n: u8 = 5; // Test input below ten
    let result = format_u8(n, &mut output); // Call the function

    assert_eq!(result, 1); // Expected return value
    assert_eq!(&output[..1], &[b'0' + n]); // Expected output for n < 10
}

#[test]
fn test_format_u8_zero() {
    let mut output = [0u8; 3]; // Output buffer
    let n: u8 = 0; // Test input zero
    let result = format_u8(n, &mut output); // Call the function

    assert_eq!(result, 1); // Expected return value
    assert_eq!(&output[..1], &[b'0']); // Expected output for n = 0
}

