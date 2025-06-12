// Answer 0

#[test]
fn test_deserialize_any_null() {
    let input = "null";
    let mut de = Deserializer::new(input.as_bytes());
    let visitor = MockVisitor {};
    let _ = de.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_true() {
    let input = "true";
    let mut de = Deserializer::new(input.as_bytes());
    let visitor = MockVisitor {};
    let _ = de.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_false() {
    let input = "false";
    let mut de = Deserializer::new(input.as_bytes());
    let visitor = MockVisitor {};
    let _ = de.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_negative_number() {
    let input = "-123.45";
    let mut de = Deserializer::new(input.as_bytes());
    let visitor = MockVisitor {};
    let _ = de.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_positive_number() {
    let input = "123.45";
    let mut de = Deserializer::new(input.as_bytes());
    let visitor = MockVisitor {};
    let _ = de.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_string() {
    let input = "\"Hello, World!\"";
    let mut de = Deserializer::new(input.as_bytes());
    let visitor = MockVisitor {};
    let _ = de.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_array() {
    let input = "[1, 2, 3]";
    let mut de = Deserializer::new(input.as_bytes());
    let visitor = MockVisitor {};
    let _ = de.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_object() {
    let input = "{\"key\": \"value\"}";
    let mut de = Deserializer::new(input.as_bytes());
    let visitor = MockVisitor {};
    let _ = de.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_whitespace_error() {
    let input = "   ";
    let mut de = Deserializer::new(input.as_bytes());
    let visitor = MockVisitor {};
    let _ = de.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_invalid_number() {
    let input = "123abc";
    let mut de = Deserializer::new(input.as_bytes());
    let visitor = MockVisitor {};
    let _ = de.deserialize_any(visitor);
}

