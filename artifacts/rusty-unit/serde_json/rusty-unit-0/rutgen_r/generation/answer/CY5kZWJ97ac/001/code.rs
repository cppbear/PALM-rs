// Answer 0

#[test]
fn test_sort_all_objects_empty_object() {
    let mut value = serde_json::Value::Object(serde_json::Map::new());
    value.sort_all_objects();
    assert_eq!(value, serde_json::Value::Object(serde_json::Map::new()));
}

#[test]
fn test_sort_all_objects_single_key_object() {
    let mut value = serde_json::Value::Object(serde_json::Map::from_iter(vec![
        (String::from("b"), serde_json::Value::from(2)),
    ]));
    value.sort_all_objects();
    let expected = serde_json::Value::Object(serde_json::Map::from_iter(vec![
        (String::from("b"), serde_json::Value::from(2)),
    ]));
    assert_eq!(value, expected);
}

#[test]
fn test_sort_all_objects_multiple_keys_object() {
    let mut value = serde_json::Value::Object(serde_json::Map::from_iter(vec![
        (String::from("b"), serde_json::Value::from(2)),
        (String::from("a"), serde_json::Value::from(1)),
    ]));
    value.sort_all_objects();
    let expected = serde_json::Value::Object(serde_json::Map::from_iter(vec![
        (String::from("a"), serde_json::Value::from(1)),
        (String::from("b"), serde_json::Value::from(2)),
    ]));
    assert_eq!(value, expected);
}

#[test]
fn test_sort_all_objects_nested_objects() {
    let mut value = serde_json::Value::Object(serde_json::Map::from_iter(vec![
        (String::from("outer"), serde_json::Value::Object(serde_json::Map::from_iter(vec![
            (String::from("b"), serde_json::Value::from(2)),
            (String::from("a"), serde_json::Value::from(1)),
        ]))),
    ]));
    value.sort_all_objects();
    let expected = serde_json::Value::Object(serde_json::Map::from_iter(vec![
        (String::from("outer"), serde_json::Value::Object(serde_json::Map::from_iter(vec![
            (String::from("a"), serde_json::Value::from(1)),
            (String::from("b"), serde_json::Value::from(2)),
        ]))),
    ]));
    assert_eq!(value, expected);
}

#[test]
fn test_sort_all_objects_with_arrays() {
    let mut value = serde_json::Value::Array(vec![
        serde_json::Value::Object(serde_json::Map::from_iter(vec![
            (String::from("b"), serde_json::Value::from(2)),
            (String::from("a"), serde_json::Value::from(1)),
        ])),
        serde_json::Value::Object(serde_json::Map::from_iter(vec![
            (String::from("c"), serde_json::Value::from(3)),
        ])),
    ]);
    value.sort_all_objects();
    let expected = serde_json::Value::Array(vec![
        serde_json::Value::Object(serde_json::Map::from_iter(vec![
            (String::from("a"), serde_json::Value::from(1)),
            (String::from("b"), serde_json::Value::from(2)),
        ])),
        serde_json::Value::Object(serde_json::Map::from_iter(vec![
            (String::from("c"), serde_json::Value::from(3)),
        ])),
    ]);
    assert_eq!(value, expected);
}

#[test]
#[should_panic]
fn test_sort_all_objects_panics_on_non_object() {
    let mut value = serde_json::Value::Number(serde_json::Number::from(42));
    value.sort_all_objects();
}

