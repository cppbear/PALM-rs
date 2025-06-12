// Answer 0

#[test]
fn test_display_value_compact() {
    use serde_json::Value;
    use serde_json::Number;
    use alloc::string::String;
    use alloc::vec::Vec;

    let json = Value::Object(Map {
        map: MapImpl::new()
            .insert(String::from("city"), Value::String(String::from("London")))
            .insert(String::from("street"), Value::String(String::from("10 Downing Street"))),
    });

    let compact = format!("{}", json);
    assert_eq!(compact, "{\"city\":\"London\",\"street\":\"10 Downing Street\"}");
}

#[test]
fn test_display_value_pretty() {
    use serde_json::Value;
    use serde_json::Number;
    use alloc::string::String;
    use alloc::vec::Vec;

    let json = Value::Object(Map {
        map: MapImpl::new()
            .insert(String::from("city"), Value::String(String::from("London")))
            .insert(String::from("street"), Value::String(String::from("10 Downing Street"))),
    });

    let pretty = format!("{:#}", json);
    assert_eq!(pretty, "{\n  \"city\": \"London\",\n  \"street\": \"10 Downing Street\"\n}");
}

#[test]
#[should_panic]
fn test_display_value_null() {
    use serde_json::Value;

    let json = Value::Null;
    let _ = format!("{}", json);
}

