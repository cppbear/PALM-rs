// Answer 0

#[test]
fn test_byte_input_len_non_empty() {
    let input_data: &[u8] = b"Hello, world!";
    let byte_input = ByteInput {
        text: input_data,
        only_utf8: true,
    };
    assert_eq!(byte_input.len(), 13);
}

#[test]
fn test_byte_input_len_empty() {
    let input_data: &[u8] = b"";
    let byte_input = ByteInput {
        text: input_data,
        only_utf8: true,
    };
    assert_eq!(byte_input.len(), 0);
}

#[test]
fn test_byte_input_len_single_character() {
    let input_data: &[u8] = b"A";
    let byte_input = ByteInput {
        text: input_data,
        only_utf8: true,
    };
    assert_eq!(byte_input.len(), 1);
}

#[test]
fn test_byte_input_len_special_characters() {
    let input_data: &[u8] = b"!@#$%^&*()";
    let byte_input = ByteInput {
        text: input_data,
        only_utf8: true,
    };
    assert_eq!(byte_input.len(), 10);
}

#[test]
fn test_byte_input_len_utf8_characters() {
    let input_data: &[u8] = "こんにちは".as_bytes(); // UTF-8 characters
    let byte_input = ByteInput {
        text: input_data,
        only_utf8: true,
    };
    assert_eq!(byte_input.len(), 15); // "こんにちは" is 15 bytes in UTF-8
}

