// Answer 0

#[test]
fn test_string_deserializer_empty() {
    let value = String::from("");
    let deserializer = StringDeserializer::new(value);
}

#[test]
fn test_string_deserializer_lowercase() {
    let value = String::from("a");
    let deserializer = StringDeserializer::new(value);
}

#[test]
fn test_string_deserializer_uppercase() {
    let value = String::from("A");
    let deserializer = StringDeserializer::new(value);
}

#[test]
fn test_string_deserializer_numeric() {
    let value = String::from("0");
    let deserializer = StringDeserializer::new(value);
}

#[test]
fn test_string_deserializer_long_string() {
    let value = "x".repeat(2u16.into().pow(16)); // generates a string of 2^16 characters
    let deserializer = StringDeserializer::new(value);
}

