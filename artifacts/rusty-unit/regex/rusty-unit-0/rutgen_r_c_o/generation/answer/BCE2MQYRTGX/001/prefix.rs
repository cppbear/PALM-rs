// Answer 0

#[test]
fn test_set_word_minimum_value() {
    let mut flags = StateFlags(0);
    flags.set_word();
}

#[test]
fn test_set_word_boundary_value_1() {
    let mut flags = StateFlags(1);
    flags.set_word();
}

#[test]
fn test_set_word_boundary_value_2() {
    let mut flags = StateFlags(2);
    flags.set_word();
}

#[test]
fn test_set_word_maximum_value() {
    let mut flags = StateFlags(255);
    flags.set_word();
}

#[test]
fn test_set_word_middle_value() {
    let mut flags = StateFlags(127);
    flags.set_word();
}

#[test]
fn test_set_word_value_already_set() {
    let mut flags = StateFlags(0b000000_1_0);
    flags.set_word();
} 

#[test]
fn test_set_word_value_before_setting() {
    let mut flags = StateFlags(0b000000_0_0);
    flags.set_word();
}

#[test]
fn test_set_word_with_high_bits() {
    let mut flags = StateFlags(0b11111111);
    flags.set_word();
}

