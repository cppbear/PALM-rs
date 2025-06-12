// Answer 0

#[test]
fn test_as_str_empty() {
    let data: [u8; 15] = [b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let len: u8 = 0;
    let inline_extension = InlineExtension(data, len);
    inline_extension.as_str();
}

#[test]
fn test_as_str_length_one() {
    let data: [u8; 15] = [b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let len: u8 = 1;
    let inline_extension = InlineExtension(data, len);
    inline_extension.as_str();
}

#[test]
fn test_as_str_length_fifteen() {
    let data: [u8; 15] = [b'H', b'e', b'l', b'l', b'o', b' ', b'W', b'o', b'r', b'l', b'd', b'!', 0, 0, 0];
    let len: u8 = 12;
    let inline_extension = InlineExtension(data, len);
    inline_extension.as_str();
}

#[test]
fn test_as_str_length_max() {
    let data: [u8; 15] = [b'R', b'u', b's', b't', b' ', b't', b'e', b's', b't', b' ', b'm', b'e', b't', b'h', b'o', b'd'];
    let len: u8 = 15;
    let inline_extension = InlineExtension(data, len);
    inline_extension.as_str();
}

#[test]
#[should_panic]
fn test_as_str_invalid_utf8() {
    let data: [u8; 15] = [b'\xFF', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let len: u8 = 1;
    let inline_extension = InlineExtension(data, len);
    inline_extension.as_str();
}

