// Answer 0

#[test]
fn test_to_string_pretty_with_struct() {
    use serde::ser::Serialize;

    #[derive(Serialize)]
    struct TestStruct {
        name: String,
        age: u32,
    }

    let test_data = TestStruct {
        name: String::from("Alice"),
        age: 30,
    };

    let result = to_string_pretty(&test_data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{"name":"Alice","age":30}"#);
}

#[test]
fn test_to_string_pretty_with_empty_struct() {
    use serde::ser::Serialize;

    #[derive(Serialize)]
    struct EmptyStruct;

    let test_data = EmptyStruct;

    let result = to_string_pretty(&test_data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{}"#);
}

#[test]
fn test_to_string_pretty_with_map() {
    use serde::ser::Serialize;
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);

    let result = to_string_pretty(&map);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{"key1":1,"key2":2}"#);
}

#[test]
#[should_panic]
fn test_to_string_pretty_with_non_string_map_keys() {
    use serde::ser::Serialize;
    use std::collections::HashMap;

    let mut map: HashMap<u32, String> = HashMap::new();
    map.insert(1, String::from("value1"));

    let _result = to_string_pretty(&map);
}

