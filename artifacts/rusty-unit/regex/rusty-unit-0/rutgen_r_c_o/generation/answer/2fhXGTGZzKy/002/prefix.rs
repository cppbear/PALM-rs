// Answer 0

#[test]
fn test_escape_into_empty_text() {
    let mut buf = String::new();
    escape_into("", &mut buf);
}

#[test]
fn test_escape_into_no_meta_characters() {
    let mut buf = String::new();
    escape_into("Hello, World!", &mut buf);
}

#[test]
fn test_escape_into_with_one_meta_character() {
    let mut buf = String::new();
    escape_into("Hello, (World)!", &mut buf);
}

#[test]
fn test_escape_into_multiple_meta_characters() {
    let mut buf = String::new();
    escape_into("Hello, +World*!", &mut buf);
}

#[test]
fn test_escape_into_all_meta_characters() {
    let mut buf = String::new();
    escape_into(".*?+|()[]{}^$&-~", &mut buf);
}

#[test]
fn test_escape_into_long_text() {
    let mut buf = String::new();
    escape_into("This is a test with a meta character: &!", &mut buf);
}

#[test]
fn test_escape_into_full_buffer() {
    let mut buf = String::with_capacity(128);
    escape_into("This text could potentially overflow in some cases.", &mut buf);
}

