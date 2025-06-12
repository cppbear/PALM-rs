// Answer 0

#[test]
fn test_sort_all_objects_empty_object() {
    let mut value = Value::Object(Map::new());
    value.sort_all_objects();
}

#[test]
fn test_sort_all_objects_empty_array() {
    let mut value = Value::Array(Vec::new());
    value.sort_all_objects();
}

#[test]
fn test_sort_all_objects_single_key_object() {
    let mut value = Value::Object(Map::from([(String::from("b"), Value::String(String::from("value")))]));
    value.sort_all_objects();
}

#[test]
fn test_sort_all_objects_two_key_object() {
    let mut value = Value::Object(Map::from([
        (String::from("b"), Value::String(String::from("value_b"))),
        (String::from("a"), Value::String(String::from("value_a"))),
    ]));
    value.sort_all_objects();
}

#[test]
fn test_sort_all_objects_nested_objects() {
    let mut value = Value::Object(Map::from([
        (String::from("b"), Value::Object(Map::from([(String::from("d"), Value::Null)]))),
        (String::from("a"), Value::Object(Map::from([(String::from("c"), Value::Bool(true))]))),
    ]));
    value.sort_all_objects();
}

#[test]
fn test_sort_all_objects_nested_arrays() {
    let mut value = Value::Array(vec![
        Value::Array(vec![Value::String(String::from("item_2")), Value::String(String::from("item_1"))]),
        Value::Array(vec![Value::String(String::from("item_4")), Value::String(String::from("item_3"))]),
    ]);
    value.sort_all_objects();
}

#[test]
fn test_sort_all_objects_multiple_levels() {
    let mut value = Value::Object(Map::from([
        (String::from("c"), Value::Array(vec![Value::Number(Number { n: 3 })])),
        (String::from("a"), Value::Object(Map::from([
            (String::from("b"), Value::Bool(true)),
            (String::from("a"), Value::Null)
        ]))),
    ]));
    value.sort_all_objects();
}

#[test]
fn test_sort_all_objects_large_object() {
    let mut map = Map::new();
    for i in 0..100 {
        map.insert(format!("key_{}", 100 - i), Value::String(format!("value_{}", i)));
    }
    let mut value = Value::Object(map);
    value.sort_all_objects();
}

#[test]
fn test_sort_all_objects_large_nested_structure() {
    let mut value = Value::Object(Map::from([
        (String::from("z"), Value::Array(vec![Value::String(String::from("end"))])),
        (String::from("a"), Value::Object(Map::from([
            (String::from("b"), Value::Object(Map::from([
                (String::from("c"), Value::String(String::from("nested"))),
            ]))),
        ]))),
    ]));
    value.sort_all_objects();
}

#[test]
fn test_sort_all_objects_with_various_types() {
    let mut value = Value::Object(Map::from([
        (String::from("number"), Value::Number(Number { n: 42 })),
        (String::from("string"), Value::String(String::from("hello"))),
        (String::from("bool"), Value::Bool(true)),
        (String::from("null"), Value::Null),
    ]));
    value.sort_all_objects();
}

