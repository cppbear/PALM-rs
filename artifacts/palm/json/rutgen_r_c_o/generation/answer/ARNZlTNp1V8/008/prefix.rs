// Answer 0

#[test]
fn test_deserialize_any_unit() {
    let input = b"null";
    let mut cursor = Cursor::new(input);
    let mut deserializer = Deserializer::new(&mut cursor);
    let visitor = UnitVisitor;
    deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_bool_true() {
    let input = b"true";
    let mut cursor = Cursor::new(input);
    let mut deserializer = Deserializer::new(&mut cursor);
    let visitor = BoolVisitor;
    deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_bool_false() {
    let input = b"false";
    let mut cursor = Cursor::new(input);
    let mut deserializer = Deserializer::new(&mut cursor);
    let visitor = BoolVisitor;
    deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_negative_number() {
    let input = b"-42";
    let mut cursor = Cursor::new(input);
    let mut deserializer = Deserializer::new(&mut cursor);
    let visitor = I64Visitor;
    deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_positive_number() {
    let input = b"123";
    let mut cursor = Cursor::new(input);
    let mut deserializer = Deserializer::new(&mut cursor);
    let visitor = U64Visitor;
    deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_string() {
    let input = b"\"Hello, World!\"";
    let mut cursor = Cursor::new(input);
    let mut deserializer = Deserializer::new(&mut cursor);
    let visitor = StrVisitor;
    deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_array() {
    let input = b"[1, 2, 3]";
    let mut cursor = Cursor::new(input);
    let mut deserializer = Deserializer::new(&mut cursor);
    let visitor = SeqVisitor;
    deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_object() {
    let input = b"{\"key\": \"value\"}";
    let mut cursor = Cursor::new(input);
    let mut deserializer = Deserializer::new(&mut cursor);
    let visitor = MapVisitor;
    deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_invalid_whitespace() {
    let input = b"   ";
    let mut cursor = Cursor::new(input);
    let mut deserializer = Deserializer::new(&mut cursor);
    let visitor = InvalidVisitor;
    deserializer.deserialize_any(visitor);
}

#[test]
#[should_panic]
fn test_deserialize_any_out_of_bounds() {
    let input = b"99999999999999999999999999999999999999999999999999999999999999999999999999999999";
    let mut cursor = Cursor::new(input);
    let mut deserializer = Deserializer::new(&mut cursor);
    let visitor = I64Visitor;
    deserializer.deserialize_any(visitor);
}

