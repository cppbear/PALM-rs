// Answer 0

#[test]
fn test_new_with_reserved_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/="; // includes '=' which is PAD_BYTE
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_new_with_unique_printable_bytes_including_pad_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/ABCDEFGHIJKLMNOPQRSTUVWXYZ"; // includes '=' which is PAD_BYTE
    let result = Alphabet::new(alphabet);
}

