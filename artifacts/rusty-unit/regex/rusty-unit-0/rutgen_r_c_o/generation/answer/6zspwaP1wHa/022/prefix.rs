// Answer 0

#[test]
fn test_is_ascii_word_b_255() {
    let byte_instance = Byte(255);
    byte_instance.is_ascii_word();
}

#[test]
fn test_is_ascii_word_b_256() {
    let byte_instance = Byte(256);
    byte_instance.is_ascii_word();
}

#[test]
fn test_is_ascii_word_b_127() {
    let byte_instance = Byte(127);
    byte_instance.is_ascii_word();
}

#[test]
fn test_is_ascii_word_b_128() {
    let byte_instance = Byte(128);
    byte_instance.is_ascii_word();
}

