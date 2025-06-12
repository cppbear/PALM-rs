// Answer 0

#[test]
fn test_into_iter_with_valid_json() {
    use serde_json::Deserializer;
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Debug, Deserialize)]
    struct TestStruct {
        value: i32,
    }

    let json_data = r#"[{"value":1}, {"value":2}, {"value":3}]"#;
    let cursor = Cursor::new(json_data);
    let deserializer = Deserializer::from_reader(cursor);
    let mut iter = deserializer.into_iter::<TestStruct>();

    assert_eq!(iter.next().unwrap().value, 1);
    assert_eq!(iter.next().unwrap().value, 2);
    assert_eq!(iter.next().unwrap().value, 3);
    assert!(iter.next().is_none());
}

#[test]
#[should_panic]
fn test_into_iter_with_invalid_json() {
    use serde_json::Deserializer;
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Debug, Deserialize)]
    struct TestStruct {
        value: i32,
    }

    let json_data = r#"[{"value":1}, {"value":"invalid"}, {"value":3}]"#;
    let cursor = Cursor::new(json_data);
    let deserializer = Deserializer::from_reader(cursor);
    let mut iter = deserializer.into_iter::<TestStruct>();

    // This should cause a panic as the second value cannot be deserialized
    let _ = iter.next();
    let _ = iter.next();
}

