// Answer 0

#[test]
#[should_panic]
fn test_invalid_utf8_sequence_one_byte() {
    let bytes = Bytes::from_static(&[0xFF]);
    unsafe { from_utf8_unchecked(bytes) };
}

#[test]
#[should_panic]
fn test_invalid_utf8_sequence_two_bytes() {
    let bytes = Bytes::from_static(&[0xC3, 0x28]);
    unsafe { from_utf8_unchecked(bytes) };
}

#[test]
#[should_panic]
fn test_invalid_utf8_sequence_four_bytes() {
    let bytes = Bytes::from_static(&[0xF0, 0x9D, 0x80, 0x28]);
    unsafe { from_utf8_unchecked(bytes) };
}

#[test]
#[should_panic]
fn test_invalid_utf8_sequence_three_bytes() {
    let bytes = Bytes::from_static(&[0xE2, 0x28, 0xA1]);
    unsafe { from_utf8_unchecked(bytes) };
}

#[test]
#[should_panic]
fn test_invalid_utf8_sequence_mixed() {
    let bytes = Bytes::from_static(&[0x80, 0xFF, 0xC2]);
    unsafe { from_utf8_unchecked(bytes) };
}

#[test]
#[should_panic]
fn test_invalid_utf8_sequence_long() {
    let bytes = Bytes::from_static(&[0xE2, 0x82, 0x28, 0xFF, 0x3F]);
    unsafe { from_utf8_unchecked(bytes) };
}

#[test]
#[should_panic]
fn test_invalid_utf8_sequence_length_limit() {
    let bytes = Bytes::from_static(&[0xFE, 0xFD, 0xFB, 0xF8, 0xF0]);
    unsafe { from_utf8_unchecked(bytes) };
}

#[test]
#[should_panic]
fn test_invalid_utf8_sequence_random_bytes() {
    let bytes = Bytes::from_static(&[0x80, 0xFF, 0xC1, 0xF0, 0xA1, 0xB0]);
    unsafe { from_utf8_unchecked(bytes) };
}

