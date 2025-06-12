// Answer 0

#[test]
fn test_as_array_mut_with_null() {
    let mut v = Value::Null;
    v.as_array_mut();
}

#[test]
fn test_as_array_mut_with_bool_false() {
    let mut v = Value::Bool(false);
    v.as_array_mut();
}

#[test]
fn test_as_array_mut_with_number() {
    let mut v = Value::Number(Number { n: 0 });
    v.as_array_mut();
}

#[test]
fn test_as_array_mut_with_string() {
    let mut v = Value::String(String::from("a string"));
    v.as_array_mut();
}

#[test]
fn test_as_array_mut_with_object() {
    let mut v = Value::Object(Map { map: MapImpl::new() });
    v.as_array_mut();
}

