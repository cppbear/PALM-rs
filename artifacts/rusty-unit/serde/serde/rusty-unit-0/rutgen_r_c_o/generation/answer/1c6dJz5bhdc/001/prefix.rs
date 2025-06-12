// Answer 0

#[test]
fn test_bad_type_boolean() {
    let input = Unsupported::Boolean;
    let _ = FlatMapSerializer::bad_type(input);
}

#[test]
fn test_bad_type_integer() {
    let input = Unsupported::Integer;
    let _ = FlatMapSerializer::bad_type(input);
}

#[test]
fn test_bad_type_float() {
    let input = Unsupported::Float;
    let _ = FlatMapSerializer::bad_type(input);
}

#[test]
fn test_bad_type_char() {
    let input = Unsupported::Char;
    let _ = FlatMapSerializer::bad_type(input);
}

#[test]
fn test_bad_type_string() {
    let input = Unsupported::String;
    let _ = FlatMapSerializer::bad_type(input);
}

#[test]
fn test_bad_type_byte_array() {
    let input = Unsupported::ByteArray;
    let _ = FlatMapSerializer::bad_type(input);
}

#[test]
fn test_bad_type_optional() {
    let input = Unsupported::Optional;
    let _ = FlatMapSerializer::bad_type(input);
}

#[test]
fn test_bad_type_sequence() {
    let input = Unsupported::Sequence;
    let _ = FlatMapSerializer::bad_type(input);
}

#[test]
fn test_bad_type_tuple() {
    let input = Unsupported::Tuple;
    let _ = FlatMapSerializer::bad_type(input);
}

#[test]
fn test_bad_type_tuple_struct() {
    let input = Unsupported::TupleStruct;
    let _ = FlatMapSerializer::bad_type(input);
}

