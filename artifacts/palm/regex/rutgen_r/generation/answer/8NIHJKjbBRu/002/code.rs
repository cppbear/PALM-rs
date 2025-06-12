// Answer 0

#[test]
fn test_next_utf8_valid_sequence_3_byte() {
    let text: &[u8] = &[0b110_11111, 0b101_00000, 0b101_00001]; // 3-byte UTF-8 sequence starts with 0b110_11111
    let index = 0; // Starting index
    let result = next_utf8(text, index);
    assert_eq!(result, index + 2); // Since 0b110_11111 is 2 bytes; expect index + 2
}

#[test]
fn test_next_utf8_out_of_bounds() {
    let text: &[u8] = &[0b110_11111, 0b101_00000];
    let index = 2; // Out of bounds index, should trigger the panic condition
    let result = next_utf8(text, index);
    assert_eq!(result, index + 1); // Expect going out of bounds returns index + 1
} 

#[test]
fn test_next_utf8_valid_sequence_4_byte() {
    let text: &[u8] = &[0b1110_1111, 0b101_00000, 0b101_00001, 0b101_00010]; // 4-byte UTF-8 sequence starts with 0b1110_1111
    let index = 0; // Starting index
    let result = next_utf8(text, index);
    assert_eq!(result, index + 3); // Expect index + 3 since it is a 4-byte sequence
} 

#[test]
fn test_next_utf8_valid_sequence_2_byte() {
    let text: &[u8] = &[0b110_00010, 0b101_00000]; // 2-byte UTF-8 sequence starting with 0b110_00010
    let index = 0; // Starting index
    let result = next_utf8(text, index);
    assert_eq!(result, index + 2); // Expect index + 2 since it's a 2-byte sequence
}

