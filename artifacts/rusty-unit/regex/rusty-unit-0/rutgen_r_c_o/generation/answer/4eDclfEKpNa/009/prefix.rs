// Answer 0

#[test]
fn test_is_word_byte_case_a() {
    is_word_byte(b'a');
}

#[test]
fn test_is_word_byte_case_b() {
    is_word_byte(b'z');
}

#[test]
fn test_is_word_byte_case_c() {
    is_word_byte(b'_');
}

#[test]
fn test_is_word_byte_case_d() {
    is_word_byte(b'b');
}

#[test]
fn test_is_word_byte_case_e() {
    is_word_byte(b'y');
}

#[test]
fn test_is_word_byte_case_f() {
    is_word_byte(b'A');
}

#[test]
fn test_is_word_byte_case_g() {
    is_word_byte(b'Z');
}

