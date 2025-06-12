// Answer 0

fn make_error(msg: String) -> Error {
    // Assume this returns an Error struct from a custom error handling library
    Error(msg)
}

struct Error(String);

use std::fmt::Display;

fn custom<T: Display>(msg: T) -> Error {
    make_error(msg.to_string())
}

#[test]
fn test_custom_with_string() {
    let input = "Hello, world!";
    let error = custom(input);
    assert_eq!(error.0, "Hello, world!");
}

#[test]
fn test_custom_with_integer() {
    let input = 42;
    let error = custom(input);
    assert_eq!(error.0, "42");
}

#[test]
fn test_custom_with_float() {
    let input = 3.14;
    let error = custom(input);
    assert_eq!(error.0, "3.14");
}

#[test]
fn test_custom_with_char() {
    let input = 'A';
    let error = custom(input);
    assert_eq!(error.0, "A");
}

#[test]
fn test_custom_with_string_slice() {
    let input: &str = "A string slice";
    let error = custom(input);
    assert_eq!(error.0, "A string slice");
}

#[test]
#[should_panic]
fn test_custom_with_non_display() {
    struct NoDisplay;
    let _ = custom(NoDisplay);
}

