// Answer 0

#[test]
fn test_with_value_integer() {
    let cell = OnceCell::with_value(42);
}

#[test]
fn test_with_value_float() {
    let cell = OnceCell::with_value(3.14);
}

#[test]
fn test_with_value_string() {
    let cell = OnceCell::with_value(String::from("Hello, World!"));
}

#[test]
fn test_with_value_empty_string() {
    let cell = OnceCell::with_value(String::from(""));
}

#[test]
fn test_with_value_large_integer() {
    let cell = OnceCell::with_value(1_000_000);
}

#[test]
fn test_with_value_tuple() {
    let cell = OnceCell::with_value((1, 2, 3));
}

#[test]
fn test_with_value_boolean() {
    let cell = OnceCell::with_value(true);
}

#[test]
fn test_with_value_array() {
    let cell = OnceCell::with_value([1, 2, 3, 4, 5]);
}

#[test]
fn test_with_value_character() {
    let cell = OnceCell::with_value('A');
}

#[test]
fn test_with_value_large_string() {
    let cell = OnceCell::with_value(String::from("a".repeat(10_000)));
}

