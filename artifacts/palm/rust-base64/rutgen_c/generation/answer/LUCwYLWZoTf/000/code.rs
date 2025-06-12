// Answer 0

#[test]
fn test_consume_string() {
    let mut string_consumer = String::new();
    string_consumer.consume("Hello, ");
    string_consumer.consume("world!");
    assert_eq!(string_consumer, "Hello, world!");
}

#[test]
fn test_consume_empty_string() {
    let mut string_consumer = String::new();
    string_consumer.consume("");
    assert_eq!(string_consumer, "");
}

#[test]
fn test_consume_special_characters() {
    let mut string_consumer = String::new();
    string_consumer.consume("!@#$%^&*()");
    assert_eq!(string_consumer, "!@#$%^&*()");
}

#[test]
fn test_consume_multiple_consume_calls() {
    let mut string_consumer = String::new();
    string_consumer.consume("First part. ");
    string_consumer.consume("Second part.");
    assert_eq!(string_consumer, "First part. Second part.");
}

