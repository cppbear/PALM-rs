// Answer 0

#[test]
fn test_display_json_value_pretty_format_valid_input() {
    let json = Value::Object(Map {
        map: MapImpl::new(vec![
            (String::from("city"), Value::String(String::from("London"))),
            (String::from("street"), Value::String(String::from("10 Downing Street"))),
        ]),
    });
    let _ = format!("{:#}", json);
}

#[test]
fn test_display_json_value_pretty_format_different_city() {
    let json = Value::Object(Map {
        map: MapImpl::new(vec![
            (String::from("city"), Value::String(String::from("New York"))),
            (String::from("street"), Value::String(String::from("5th Avenue"))),
        ]),
    });
    let _ = format!("{:#}", json);
}

#[test]
fn test_display_json_value_pretty_format_empty_string_values() {
    let json = Value::Object(Map {
        map: MapImpl::new(vec![
            (String::from("city"), Value::String(String::from(""))),
            (String::from("street"), Value::String(String::from(""))),
        ]),
    });
    let _ = format!("{:#}", json);
}

#[test]
fn test_display_json_value_pretty_format_special_characters() {
    let json = Value::Object(Map {
        map: MapImpl::new(vec![
            (String::from("city"), Value::String(String::from("Café"))),
            (String::from("street"), Value::String(String::from("Baker St."))),
        ]),
    });
    let _ = format!("{:#}", json);
}

#[test]
fn test_display_json_value_pretty_format_long_strings() {
    let json = Value::Object(Map {
        map: MapImpl::new(vec![
            (String::from("city"), Value::String(String::from("A Very Long City Name That Exceeds Usual Length"))),
            (String::from("street"), Value::String(String::from("A Very Long Street Name That Exceeds Usual Length"))),
        ]),
    });
    let _ = format!("{:#}", json);
}

