// Answer 0

#[test]
fn test_push_wtf8_codepoint_boundary_conditions() {
    let mut buffer = Vec::new();

    // Test case for n == 0x80, should push the value correctly as it's the boundary
    push_wtf8_codepoint(0x80, &mut buffer);
    assert_eq!(buffer, vec![0b1100_0000, 0b1000_0000]);

    // Reset the buffer for the next test case
    buffer.clear();

    // Test case for n == 0x7FF, should push the value correctly
    push_wtf8_codepoint(0x7FF, &mut buffer);
    assert_eq!(buffer, vec![0b1100_0011, 0b1011_1111]);

    // Reset the buffer for the next test case
    buffer.clear();

    // Test case for n == 0x800, should push the value correctly
    push_wtf8_codepoint(0x800, &mut buffer);
    assert_eq!(buffer, vec![0b1110_0000, 0b1000_0000, 0b1000_0000]);

    // Reset the buffer for the next test case
    buffer.clear();

    // Test case for n == 0xFFFF, should push the value correctly
    push_wtf8_codepoint(0xFFFF, &mut buffer);
    assert_eq!(buffer, vec![0b1110_1111, 0b1011_1111, 0b1000_0000]);

    // Reset the buffer for the next test case
    buffer.clear();

    // Test case for n == 0x1_0000, should push the value correctly
    push_wtf8_codepoint(0x1_0000, &mut buffer);
    assert_eq!(buffer, vec![0b1111_0000, 0b1000_0000, 0b1000_0000, 0b1000_0000]);

    // Reset the buffer for the next test case
    buffer.clear();

    // Test case for n == 0x10_FFFF, should push the value correctly
    push_wtf8_codepoint(0x10_FFFF, &mut buffer);
    assert_eq!(buffer, vec![0b1111_0001, 0b1011_1111, 0b1011_1111, 0b1000_0000]);

    // Reset the buffer for the next test case
    buffer.clear();

    // Test case for n == 0x11_0000, should panic
    let result = std::panic::catch_unwind(|| {
        push_wtf8_codepoint(0x11_0000, &mut buffer);
    });
    assert!(result.is_err());
}

