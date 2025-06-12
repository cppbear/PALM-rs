// Answer 0

#[test]
fn test_to_string_pretty_with_struct() {
    #[derive(serde::Serialize)]
    struct TestStruct {
        name: String,
        age: u32,
    }
    
    let test_value = TestStruct {
        name: String::from("Alice"),
        age: 30,
    };

    let result = to_string_pretty(&test_value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{"name":"Alice","age":30}"#);
}

#[test]
fn test_to_string_pretty_with_empty_struct() {
    #[derive(serde::Serialize)]
    struct EmptyStruct;

    let test_value = EmptyStruct;

    let result = to_string_pretty(&test_value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{}"#);
}

#[test]
fn test_to_string_pretty_with_vec() {
    let test_value = vec![1, 2, 3, 4, 5];

    let result = to_string_pretty(&test_value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"[1,2,3,4,5]"#);
}

#[test]
fn test_to_string_pretty_with_map() {
    use std::collections::HashMap;
    
    let mut test_value = HashMap::new();
    test_value.insert("key1", "value1");
    test_value.insert("key2", "value2");

    let result = to_string_pretty(&test_value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{"key1":"value1","key2":"value2"}"#);
}

#[test]
fn test_to_string_pretty_with_nested_structs() {
    #[derive(serde::Serialize)]
    struct Address {
        street: String,
        city: String,
    }

    #[derive(serde::Serialize)]
    struct Person {
        name: String,
        address: Address,
    }

    let test_value = Person {
        name: String::from("Bob"),
        address: Address {
            street: String::from("123 Main St"),
            city: String::from("Wonderland"),
        },
    };

    let result = to_string_pretty(&test_value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{"name":"Bob","address":{"street":"123 Main St","city":"Wonderland"}}"#);
}

