// Answer 0

#[test]
fn test_value_fmt_null() {
    let value = Value::Null;
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    value.fmt(formatter).unwrap();
    assert_eq!(buffer, "Null");
}

#[test]
fn test_value_fmt_bool() {
    let value_true = Value::Bool(true);
    let value_false = Value::Bool(false);
    let mut buffer_true = String::new();
    let mut buffer_false = String::new();
    let formatter_true = &mut fmt::Formatter::new(&mut buffer_true);
    let formatter_false = &mut fmt::Formatter::new(&mut buffer_false);
    value_true.fmt(formatter_true).unwrap();
    value_false.fmt(formatter_false).unwrap();
    assert_eq!(buffer_true, "Bool(true)");
    assert_eq!(buffer_false, "Bool(false)");
}

#[test]
fn test_value_fmt_number() {
    struct TestNumber; // Creating a struct to implement the Number trait
    impl Number for TestNumber {} // Implementing the trait on the struct
    let number_value = Value::Number(Number { n: TestNumber });
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    number_value.fmt(formatter).unwrap();
    // Assuming the expected output when formatting TestNumber
    assert_eq!(buffer, "TestNumber"); // This will depend on how Debug is implemented for TestNumber
}

#[test]
fn test_value_fmt_string() {
    let value = Value::String(String::from("a string"));
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    value.fmt(formatter).unwrap();
    assert_eq!(buffer, "String(\"a string\")");
}

#[test]
fn test_value_fmt_array() {
    let array_value = Value::Array(vec![Value::String(String::from("item1")), Value::String(String::from("item2"))]);
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    array_value.fmt(formatter).unwrap();
    assert_eq!(buffer, "Array [String(\"item1\"), String(\"item2\")]");
}

#[test]
fn test_value_fmt_object() {
    let mut object_map = Map::new();
    object_map.insert(String::from("key1"), Value::String(String::from("value1")));
    let object_value = Value::Object(object_map);
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    object_value.fmt(formatter).unwrap();
    // Depending on how Map's Debug trait is implemented, you would adjust the expected output
    assert_eq!(buffer, "Object { key1: String(\"value1\") }"); // This will depend on how Debug is implemented for Map
}

