// Answer 0

#[test]
fn test_deserialize_number_positive_integers() {
    let input = b"42";
    let mut deserializer = Deserializer::new(input);
    deserializer.deserialize_number(MyVisitor);
}

#[test]
fn test_deserialize_number_negative_integers() {
    let input = b"-7";
    let mut deserializer = Deserializer::new(input);
    deserializer.deserialize_number(MyVisitor);
}

#[test]
fn test_deserialize_number_zero() {
    let input = b"0";
    let mut deserializer = Deserializer::new(input);
    deserializer.deserialize_number(MyVisitor);
}

#[test]
fn test_deserialize_number_invalid_characters() {
    let input = b"notANumber";
    let mut deserializer = Deserializer::new(input);
    deserializer.deserialize_number(MyVisitor); // expects an error
}

#[test]
fn test_deserialize_number_empty_input() {
    let input = b"";
    let mut deserializer = Deserializer::new(input);
    deserializer.deserialize_number(MyVisitor); // expects an error
}

#[test]
fn test_deserialize_number_negative_character() {
    let input = b"-";
    let mut deserializer = Deserializer::new(input);
    deserializer.deserialize_number(MyVisitor); // expects an error
}

#[test]
fn test_deserialize_number_leading_zero() {
    let input = b"0123";
    let mut deserializer = Deserializer::new(input);
    deserializer.deserialize_number(MyVisitor); // expects an error
}

#[test]
fn test_deserialize_number_invalid_negative() {
    let input = b"-abc";
    let mut deserializer = Deserializer::new(input);
    deserializer.deserialize_number(MyVisitor); // expects an error
}

#[test]
fn test_deserialize_number_large_integer() {
    let input = b"1234567890";
    let mut deserializer = Deserializer::new(input);
    deserializer.deserialize_number(MyVisitor);
}

#[test]
fn test_deserialize_number_single_digit() {
    let input = b"8";
    let mut deserializer = Deserializer::new(input);
    deserializer.deserialize_number(MyVisitor);
}

