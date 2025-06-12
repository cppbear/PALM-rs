// Answer 0

#[test]
fn test_as_object_mut_non_empty() {
    let mut v = Value::Object(Map {
        map: vec![
            (String::from("key1"), Value::String(String::from("value1"))),
            (String::from("key2"), Value::Number(Number { n: 42 })),
            (String::from("key3"), Value::Bool(true)),
        ].into_iter().collect(),
    });
    let map_mut = v.as_object_mut();
}

#[test]
fn test_as_object_mut_empty() {
    let mut v = Value::Object(Map {
        map: vec![].into_iter().collect(),
    });
    let map_mut = v.as_object_mut();
}

#[test]
fn test_as_object_mut_single_entry() {
    let mut v = Value::Object(Map {
        map: vec![
            (String::from("single_key"), Value::String(String::from("single_value"))),
        ].into_iter().collect(),
    });
    let map_mut = v.as_object_mut();
}

#[test]
fn test_as_object_mut_varied_value_types() {
    let mut v = Value::Object(Map {
        map: vec![
            (String::from("string_key"), Value::String(String::from("string_value"))),
            (String::from("number_key"), Value::Number(Number { n: 100 })),
            (String::from("bool_key"), Value::Bool(false)),
            (String::from("null_key"), Value::Null),
        ].into_iter().collect(),
    });
    let map_mut = v.as_object_mut();
}

#[test]
fn test_as_object_mut_large_key_length() {
    let mut v = Value::Object(Map {
        map: vec![
            (String::from("a_very_long_key_name_exceeding_normal_length"), Value::String(String::from("value"))),
        ].into_iter().collect(),
    });
    let map_mut = v.as_object_mut();
}

#[test]
fn test_as_object_mut_nested_object() {
    let mut v = Value::Object(Map {
        map: vec![
            (String::from("outer_key"), Value::Object(Map {
                map: vec![
                    (String::from("inner_key"), Value::Number(Number { n: 3 })),
                ].into_iter().collect(),
            })),
        ].into_iter().collect(),
    });
    let map_mut = v.as_object_mut();
}

