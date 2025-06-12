// Answer 0

#[test]
fn test_serialize_some_with_i32() {
    let val = 42;
    let result = serde_json::to_value(val);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(42.into()));
}

#[test]
fn test_serialize_some_with_string() {
    let val = "hello";
    let result = serde_json::to_value(val);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::String("hello".into()));
}

#[test]
fn test_serialize_some_with_empty_vector() {
    let val: Vec<i32> = vec![];
    let result = serde_json::to_value(val);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Array(vec![]));
}

#[test]
fn test_serialize_some_with_nested_structures() {
    #[derive(Serialize)]
    struct Inner {
        id: i32,
    }

    #[derive(Serialize)]
    struct Outer {
        inner: Inner,
    }

    let val = Outer { inner: Inner { id: 1 } };
    let result = serde_json::to_value(val);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Object(
        [
            ("inner".to_string(), Value::Object(
                [("id".to_string(), Value::Number(1.into()))].iter().cloned().collect()
            ))
        ].iter().cloned().collect()
    ));
}

#[test]
fn test_serialize_some_should_panic_on_cyclic_structures() {
    #[derive(Serialize)]
    struct Node {
        value: i32,
        next: Option<Box<Node>>,
    }

    let cyclic_node = Node { value: 1, next: None };
    let mut current = &cyclic_node;
    for _ in 0..10 {
        current.next = Some(Box::new(Node { value: 1, next: None }));
        current = current.next.as_ref().unwrap();
    }
    current.next = Some(Box::new(cyclic_node)); // create cycle

    let result = std::panic::catch_unwind(|| serde_json::to_value(cyclic_node));
    assert!(result.is_err());
}

