// Answer 0

#[test]
fn test_is_escape_control_character_with_escape_characters_false() {
    let ch = 1; // control character
    let including_control_characters = false;
    is_escape(ch, including_control_characters);
}

#[test]
fn test_is_escape_control_character_with_escape_characters_true() {
    let ch = 1; // control character
    let including_control_characters = true;
    is_escape(ch, including_control_characters);
}

#[test]
fn test_is_escape_non_control_character_with_escape_characters_false() {
    let ch = 10; // control character
    let including_control_characters = false;
    is_escape(ch, including_control_characters);
}

#[test]
fn test_is_escape_non_control_character_with_escape_characters_true() {
    let ch = 10; // control character
    let including_control_characters = true;
    is_escape(ch, including_control_characters);
}

#[test]
fn test_is_escape_high_control_character_with_escape_characters_true() {
    let ch = 31; // highest control character
    let including_control_characters = true;
    is_escape(ch, including_control_characters);
}

#[test]
fn test_is_escape_non_escape_character_with_escape_characters_false() {
    let ch = 20; // a non-escape character
    let including_control_characters = false;
    is_escape(ch, including_control_characters);
}

#[test]
fn test_is_escape_non_escape_character_with_escape_characters_true() {
    let ch = 20; // a non-escape character
    let including_control_characters = true;
    is_escape(ch, including_control_characters);
}

