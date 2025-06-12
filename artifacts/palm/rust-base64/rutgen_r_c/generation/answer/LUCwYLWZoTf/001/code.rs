// Answer 0

#[test]
fn test_consume_with_non_empty_string() {
    let mut consumer: String = String::new();
    consumer.consume("Hello, ");
    consumer.consume("World!");
    assert_eq!(consumer, "Hello, World!");
}

#[test]
fn test_consume_with_empty_string() {
    let mut consumer: String = String::new();
    consumer.consume("");
    assert_eq!(consumer, "");
}

#[test]
fn test_consume_with_single_character() {
    let mut consumer: String = String::new();
    consumer.consume("A");
    assert_eq!(consumer, "A");
}

#[test]
fn test_consume_with_whitespace() {
    let mut consumer: String = String::new();
    consumer.consume("   ");
    assert_eq!(consumer, "   ");
}

#[test]
fn test_consume_multiple_times() {
    let mut consumer: String = String::new();
    consumer.consume("A");
    consumer.consume("B");
    consumer.consume("C");
    assert_eq!(consumer, "ABC");
}

#[test]
fn test_consume_with_special_characters() {
    let mut consumer: String = String::new();
    consumer.consume("!@#$%^&*()");
    assert_eq!(consumer, "!@#$%^&*()");
}

