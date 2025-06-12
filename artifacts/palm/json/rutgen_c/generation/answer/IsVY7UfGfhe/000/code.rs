// Answer 0

#[test]
fn test_serialize_simple_string() {
    #[derive(serde::Serialize)]
    struct SimpleString {
        value: String,
    }
    
    let input = SimpleString {
        value: String::from("Hello, world!"),
    };
    
    let result = to_vec(&input).unwrap();
    assert_eq!(result, b"{\"value\":\"Hello, world!\"}");
}

#[test]
fn test_serialize_empty_string() {
    #[derive(serde::Serialize)]
    struct EmptyString {
        value: String,
    }

    let input = EmptyString { value: String::new() };
    
    let result = to_vec(&input).unwrap();
    assert_eq!(result, b"{\"value\":\"\"}");
}

#[test]
fn test_serialize_integer() {
    #[derive(serde::Serialize)]
    struct SimpleInteger {
        value: i32,
    }

    let input = SimpleInteger { value: 42 };

    let result = to_vec(&input).unwrap();
    assert_eq!(result, b"{\"value\":42}");
}

#[test]
fn test_serialize_vector() {
    #[derive(serde::Serialize)]
    struct VectorExample {
        values: Vec<i32>,
    }

    let input = VectorExample {
        values: vec![1, 2, 3, 4],
    };

    let result = to_vec(&input).unwrap();
    assert_eq!(result, b"{\"values\":[1,2,3,4]}");
}

#[test]
fn test_serialize_map() {
    #[derive(serde::Serialize)]
    struct MapExample {
        items: std::collections::HashMap<String, i32>,
    }

    let mut items = std::collections::HashMap::new();
    items.insert(String::from("one"), 1);
    items.insert(String::from("two"), 2);

    let input = MapExample { items };

    let result = to_vec(&input).unwrap();
    assert_eq!(result, b"{\"items\":{\"one\":1,\"two\":2}}");
}

#[should_panic]
fn test_serialize_invalid_map() {
    #[derive(serde::Serialize)]
    struct InvalidMapExample {
        items: std::collections::HashMap<i32, i32>,
    }

    let mut items = std::collections::HashMap::new();
    items.insert(1, 100); // Non-string key, should panic

    let input = InvalidMapExample { items };
    
    let _result = to_vec(&input).unwrap(); // This should panic
}

