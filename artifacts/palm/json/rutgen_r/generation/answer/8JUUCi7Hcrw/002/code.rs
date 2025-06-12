// Answer 0

#[test]
fn test_to_vec_pretty_serializes_struct() {
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestStruct {
        field1: String,
        field2: i32,
    }

    let test_value = TestStruct {
        field1: String::from("Hello"),
        field2: 42,
    };

    let result = to_vec_pretty(&test_value);
    assert!(result.is_ok());
    let json_output = String::from_utf8(result.unwrap()).unwrap();
    assert_eq!(json_output, r#"{
  "field1": "Hello",
  "field2": 42
}"#);
}

#[test]
fn test_to_vec_pretty_serializes_empty_struct() {
    use serde::Serialize;

    #[derive(Serialize)]
    struct EmptyStruct;

    let empty_value = EmptyStruct;

    let result = to_vec_pretty(&empty_value);
    assert!(result.is_ok());
    let json_output = String::from_utf8(result.unwrap()).unwrap();
    assert_eq!(json_output, r#"{}"#);
}

#[test]
fn test_to_vec_pretty_serializes_map_with_string_keys() {
    use serde::Serialize;
    use std::collections::HashMap;

    #[derive(Serialize)]
    struct TestMap {
        map: HashMap<String, i32>,
    }

    let mut map = HashMap::new();
    map.insert(String::from("key1"), 10);
    map.insert(String::from("key2"), 20);

    let test_value = TestMap { map };

    let result = to_vec_pretty(&test_value);
    assert!(result.is_ok());
    let json_output = String::from_utf8(result.unwrap()).unwrap();
    assert_eq!(json_output, r#"{
  "map": {
    "key1": 10,
    "key2": 20
  }
}"#);
}

#[test]
#[should_panic]
fn test_to_vec_pretty_fails_on_non_string_key_in_map() {
    use serde::Serialize;
    use std::collections::HashMap;

    #[derive(Serialize)]
    struct NonStringKeyMap {
        map: HashMap<i32, i32>,
    }

    let mut map = HashMap::new();
    map.insert(1, 10);
    
    let test_value = NonStringKeyMap { map };

    // This should panic since we are using non-string keys
    let _ = to_vec_pretty(&test_value);
}

