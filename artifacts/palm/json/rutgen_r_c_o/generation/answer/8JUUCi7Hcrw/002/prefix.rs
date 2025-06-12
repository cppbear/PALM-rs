// Answer 0

#[test]
fn test_to_vec_pretty_empty_struct() {
    #[derive(serde::Serialize)]
    struct EmptyStruct;
    
    let value = EmptyStruct;
    to_vec_pretty(&value);
}

#[test]
fn test_to_vec_pretty_simple_struct() {
    #[derive(serde::Serialize)]
    struct SimpleStruct {
        name: String,
        age: u32,
    }
    
    let value = SimpleStruct {
        name: String::from("Alice"),
        age: 30,
    };
    to_vec_pretty(&value);
}

#[test]
fn test_to_vec_pretty_string_map() {
    use std::collections::HashMap;
    
    let mut value = HashMap::new();
    value.insert(String::from("key1"), String::from("value1"));
    value.insert(String::from("key2"), String::from("value2"));
    to_vec_pretty(&value);
}

#[test]
fn test_to_vec_pretty_nested_structs() {
    #[derive(serde::Serialize)]
    struct InnerStruct {
        content: String,
    }

    #[derive(serde::Serialize)]
    struct OuterStruct {
        inner: InnerStruct,
    }

    let value = OuterStruct {
        inner: InnerStruct {
            content: String::from("nested content"),
        },
    };
    to_vec_pretty(&value);
}

#[test]
fn test_to_vec_pretty_large_string() {
    let value = String::from("a".repeat(10_000));
    to_vec_pretty(&value);
}

#[test]
fn test_to_vec_pretty_large_collection() {
    let value: Vec<u32> = (0..10_000).collect();
    to_vec_pretty(&value);
}

#[test]
fn test_to_vec_pretty_array_of_primitives() {
    let value = [1, 2, 3, 4, 5];
    to_vec_pretty(&value);
}

