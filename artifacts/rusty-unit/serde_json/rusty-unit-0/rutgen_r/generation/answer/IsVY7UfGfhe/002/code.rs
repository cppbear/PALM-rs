// Answer 0

#[test]
fn test_serialize_simple_struct() {
    #[derive(serde::Serialize)]
    struct SimpleStruct {
        name: String,
        age: u32,
    }
    
    let data = SimpleStruct {
        name: String::from("Alice"),
        age: 30,
    };
    
    let result = serde_json::to_vec(&data);
    assert!(result.is_ok());
    let expected = b"{\"name\":\"Alice\",\"age\":30}";
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_serialize_empty_struct() {
    #[derive(serde::Serialize)]
    struct EmptyStruct;

    let data = EmptyStruct;
    
    let result = serde_json::to_vec(&data);
    assert!(result.is_ok());
    let expected = b"{}";
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_serialize_nested_struct() {
    #[derive(serde::Serialize)]
    struct NestedStruct {
        outer: OuterStruct,
    }

    #[derive(serde::Serialize)]
    struct OuterStruct {
        inner: String,
    }

    let data = NestedStruct {
        outer: OuterStruct {
            inner: String::from("inner value"),
        },
    };

    let result = serde_json::to_vec(&data);
    assert!(result.is_ok());
    let expected = b"{\"outer\":{\"inner\":\"inner value\"}}";
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_serialize_struct_with_vec() {
    #[derive(serde::Serialize)]
    struct VecStruct {
        items: Vec<i32>,
    }

    let data = VecStruct {
        items: vec![1, 2, 3],
    };

    let result = serde_json::to_vec(&data);
    assert!(result.is_ok());
    let expected = b"{\"items\":[1,2,3]}";
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_serialize_struct_with_map() {
    #[derive(serde::Serialize)]
    struct MapStruct {
        data: std::collections::HashMap<String, String>,
    }

    let mut map = std::collections::HashMap::new();
    map.insert(String::from("key1"), String::from("value1"));
    map.insert(String::from("key2"), String::from("value2"));

    let data = MapStruct { data: map };

    let result = serde_json::to_vec(&data);
    assert!(result.is_ok());
    let expected = br#"{"data":{"key1":"value1","key2":"value2"}}"#;
    assert_eq!(result.unwrap(), expected);
}

