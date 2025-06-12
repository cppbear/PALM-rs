// Answer 0

#[test]
fn test_at_valid_utf8_first_character() {
    let input = CharInput(b"Hello, world!");
    let result = input.at(0);
}

#[test]
fn test_at_valid_utf8_middle_character() {
    let input = CharInput(b"Hello, world!");
    let result = input.at(7);
}

#[test]
fn test_at_valid_utf8_last_character() {
    let input = CharInput(b"Hello, world!");
    let result = input.at(12);
}

#[test]
fn test_at_valid_utf8_two_byte_character() {
    let input = CharInput(b"\xC2\xA9"); // ©
    let result = input.at(0);
}

#[test]
fn test_at_valid_utf8_three_byte_character() {
    let input = CharInput(b"\xE2\x9C\x94"); // ✔
    let result = input.at(0);
}

#[test]
fn test_at_valid_utf8_four_byte_character() {
    let input = CharInput(b"\xF0\x9F\x8C\x8D"); // 🌍
    let result = input.at(0);
}

#[test]
fn test_at_empty_input() {
    let input = CharInput(b"");
    let result = input.at(0);
}

#[test]
#[should_panic]
fn test_at_out_of_bounds() {
    let input = CharInput(b"Hello, world!");
    let result = input.at(15);
}

