// Answer 0

#[test]
fn test_visit_str_empty() {
    let mut string = String::new();
    let visitor = StringInPlaceVisitor(&mut string);
    let result = visitor.visit_str("");
    assert!(result.is_ok());
    assert_eq!(*visitor.0, "");
}

#[test]
fn test_visit_str_normal() {
    let mut string = String::new();
    let visitor = StringInPlaceVisitor(&mut string);
    let result = visitor.visit_str("Hello, World!");
    assert!(result.is_ok());
    assert_eq!(*visitor.0, "Hello, World!");
}

#[test]
fn test_visit_str_with_utf8() {
    let mut string = String::new();
    let visitor = StringInPlaceVisitor(&mut string);
    let result = visitor.visit_str("こんにちは");
    assert!(result.is_ok());
    assert_eq!(*visitor.0, "こんにちは");
}

#[test]
fn test_visit_str_multiple_calls() {
    let mut string = String::new();
    let visitor = StringInPlaceVisitor(&mut string);
    let result1 = visitor.visit_str("First Call");
    assert!(result1.is_ok());
    assert_eq!(*visitor.0, "First Call");

    let result2 = visitor.visit_str("Second Call");
    assert!(result2.is_ok());
    assert_eq!(*visitor.0, "Second Call");
}

#[test]
fn test_visit_str_non_utf8() {
    let mut string = String::new();
    let visitor = StringInPlaceVisitor(&mut string);
    let invalid_bytes = [0, 159, 146, 150]; // Invalid UTF-8 bytes
    let result = visitor.visit_bytes(&invalid_bytes);
    assert!(result.is_err()); // Expecting an error since it's not a valid UTF-8 sequence
}

