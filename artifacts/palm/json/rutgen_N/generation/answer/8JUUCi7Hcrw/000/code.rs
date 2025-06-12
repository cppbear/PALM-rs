// Answer 0

#[test]
fn test_to_vec_pretty_with_simple_struct() {
    use serde::Serialize;
    use serde_json::to_vec_pretty;

    #[derive(Serialize)]
    struct SimpleStruct {
        name: String,
        age: u32,
    }

    let data = SimpleStruct {
        name: String::from("Alice"),
        age: 30,
    };

    let result = to_vec_pretty(&data).unwrap();
    let expected = br#"{
    "name": "Alice",
    "age": 30
}"#;

    assert_eq!(result, expected);
}

#[test]
fn test_to_vec_pretty_with_empty_struct() {
    use serde::Serialize;
    use serde_json::to_vec_pretty;

    #[derive(Serialize)]
    struct EmptyStruct;

    let data = EmptyStruct;

    let result = to_vec_pretty(&data).unwrap();
    let expected = br#"{}"#;

    assert_eq!(result, expected);
}

#[test]
fn test_to_vec_pretty_with_nested_struct() {
    use serde::Serialize;
    use serde_json::to_vec_pretty;

    #[derive(Serialize)]
    struct Address {
        city: String,
        country: String,
    }

    #[derive(Serialize)]
    struct User {
        name: String,
        address: Address,
    }

    let data = User {
        name: String::from("Bob"),
        address: Address {
            city: String::from("New York"),
            country: String::from("USA"),
        },
    };

    let result = to_vec_pretty(&data).unwrap();
    let expected = br#"{
    "name": "Bob",
    "address": {
        "city": "New York",
        "country": "USA"
    }
}"#;

    assert_eq!(result, expected);
}

#[test]
fn test_to_vec_pretty_with_map() {
    use serde::Serialize;
    use serde_json::to_vec_pretty;
    use std::collections::HashMap;

    #[derive(Serialize)]
    struct MapHolder {
        data: HashMap<String, String>,
    }

    let mut map = HashMap::new();
    map.insert(String::from("key1"), String::from("value1"));
    map.insert(String::from("key2"), String::from("value2"));

    let data = MapHolder { data: map };

    let result = to_vec_pretty(&data).unwrap();
    let expected = br#"{
    "data": {
        "key1": "value1",
        "key2": "value2"
    }
}"#;

    assert_eq!(result, expected);
}

#[should_panic]
#[test]
fn test_to_vec_pretty_with_non_string_keys() {
    use serde::Serialize;
    use serde_json::to_vec_pretty;
    use std::collections::HashMap;

    #[derive(Serialize)]
    struct NonStringKeyHolder {
        data: HashMap<i32, String>,
    }

    let mut map = HashMap::new();
    map.insert(1, String::from("value1"));
    map.insert(2, String::from("value2"));

    let data = NonStringKeyHolder { data: map };

    let _result = to_vec_pretty(&data).unwrap(); // This should panic
}

