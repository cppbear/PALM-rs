// Answer 0

#[test]
fn test_display_json_null() {
    let value = Value::Null;
    format!("{}", value);
}

#[test]
fn test_display_json_bool() {
    let value = Value::Bool(true);
    format!("{}", value);
}

#[test]
fn test_display_json_number() {
    let number_value = Number { n: 42 }; // Example number
    let value = Value::Number(number_value);
    format!("{}", value);
}

#[test]
fn test_display_json_string() {
    let value = Value::String(String::from("a string"));
    format!("{}", value);
}

#[test]
fn test_display_json_array() {
    let value = Value::Array(vec![
        Value::String(String::from("an")),
        Value::String(String::from("array")),
    ]);
    format!("{}", value);
}

#[test]
fn test_display_json_object() {
    let mut map = Map { map: MapImpl::new() };
    map.insert(String::from("city"), Value::String(String::from("London")));
    map.insert(String::from("street"), Value::String(String::from("10 Downing Street")));
    let value = Value::Object(map);
    format!("{}", value);
}

#[test]
fn test_display_empty_json_object() {
    let map = Map { map: MapImpl::new() };
    let value = Value::Object(map);
    format!("{}", value);
}

#[test]
fn test_display_json_nested_object() {
    let mut inner_map = Map { map: MapImpl::new() };
    inner_map.insert(String::from("inner_key"), Value::String(String::from("inner_value")));

    let mut outer_map = Map { map: MapImpl::new() };
    outer_map.insert(String::from("outer_key"), Value::Object(inner_map));
    
    let value = Value::Object(outer_map);
    format!("{}", value);
}

