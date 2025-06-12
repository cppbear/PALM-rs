// Answer 0

#[test]
fn test_sort_all_objects_single_object() {
    #[cfg(feature = "preserve_order")]
    {
        use serde_json::Value;
        let mut json_value = Value::Object(serde_json::Map::from_iter(vec![
            (String::from("b"), Value::from(2)),
            (String::from("a"), Value::from(1)),
        ]));
        json_value.sort_all_objects();
        assert_eq!(
            json_value,
            Value::Object(serde_json::Map::from_iter(vec![
                (String::from("a"), Value::from(1)),
                (String::from("b"), Value::from(2)),
            ]))
        );
    }
}

#[test]
fn test_sort_all_objects_nested_objects() {
    #[cfg(feature = "preserve_order")]
    {
        use serde_json::Value;
        let mut json_value = Value::Object(serde_json::Map::from_iter(vec![
            (String::from("c"), Value::Object(serde_json::Map::from_iter(vec![
                (String::from("b"), Value::from(2)),
                (String::from("a"), Value::from(1)),
            ]))),
            (String::from("a"), Value::from(1)),
        ]));
        json_value.sort_all_objects();
        assert_eq!(
            json_value,
            Value::Object(serde_json::Map::from_iter(vec![
                (String::from("a"), Value::from(1)),
                (String::from("c"), Value::Object(serde_json::Map::from_iter(vec![
                    (String::from("a"), Value::from(1)),
                    (String::from("b"), Value::from(2)),
                ]))),
            ]))
        );
    }
}

#[test]
fn test_sort_all_objects_array_of_objects() {
    #[cfg(feature = "preserve_order")]
    {
        use serde_json::Value;
        let mut json_value = Value::Array(vec![
            Value::Object(serde_json::Map::from_iter(vec![
                (String::from("b"), Value::from(2)),
                (String::from("a"), Value::from(1)),
            ])),
            Value::Object(serde_json::Map::from_iter(vec![
                (String::from("d"), Value::from(4)),
                (String::from("c"), Value::from(3)),
            ])),
        ]);
        json_value.sort_all_objects();
        assert_eq!(
            json_value,
            Value::Array(vec![
                Value::Object(serde_json::Map::from_iter(vec![
                    (String::from("a"), Value::from(1)),
                    (String::from("b"), Value::from(2)),
                ])),
                Value::Object(serde_json::Map::from_iter(vec![
                    (String::from("c"), Value::from(3)),
                    (String::from("d"), Value::from(4)),
                ])),
            ])
        );
    }
}

