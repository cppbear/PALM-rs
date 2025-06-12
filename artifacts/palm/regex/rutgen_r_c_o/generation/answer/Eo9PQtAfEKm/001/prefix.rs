// Answer 0

#[test]
fn test_is_word_flag_set() {
    let mut flags = StateFlags(0b00000001);
    let result = flags.is_word();
}

#[test]
fn test_is_word_flag_not_set() {
    let mut flags = StateFlags(0b00000000);
    let result = flags.is_word();
}

#[test]
fn test_is_word_flag_set_edge() {
    let mut flags = StateFlags(0b00000010);
    let result = flags.is_word();
}

#[test]
fn test_is_word_max_value() {
    let mut flags = StateFlags(255);
    let result = flags.is_word();
}

#[test]
fn test_is_word_no_flags() {
    let mut flags = StateFlags(0);
    let result = flags.is_word();
}

