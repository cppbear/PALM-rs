// Answer 0

#[test]
fn test_next_utf8_case_valid_three_byte_sequence() {
    let text = [0b1110_1111]; // Valid three-byte start
    let i = 0;
    next_utf8(&text, i);
}

#[test]
fn test_next_utf8_case_panic_out_of_bounds() {
    let text = [0b1110_1111]; // Valid three-byte start
    let i = 1; // Out of bounds index
    next_utf8(&text, i);
}

