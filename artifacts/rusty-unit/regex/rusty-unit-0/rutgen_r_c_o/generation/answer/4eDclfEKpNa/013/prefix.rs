// Answer 0

#[test]
fn test_is_word_byte_case_A() {
    let input: u8 = b'A';
    is_word_byte(input);
}

#[test]
fn test_is_word_byte_case_B() {
    let input: u8 = b'Z';
    is_word_byte(input);
}

#[test]
fn test_is_word_byte_case_C() {
    let input: u8 = b'_';
    is_word_byte(input);
}

#[test]
fn test_is_word_byte_case_D() {
    let input: u8 = 66; // 'B'
    is_word_byte(input);
}

#[test]
fn test_is_word_byte_case_E() {
    let input: u8 = 90; // 'Z'
    is_word_byte(input);
}

#[test]
fn test_is_word_byte_case_F() {
    let input: u8 = 95; // '_'
    is_word_byte(input);
}

