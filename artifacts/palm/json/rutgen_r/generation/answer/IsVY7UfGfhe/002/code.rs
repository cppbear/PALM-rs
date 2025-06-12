// Answer 0

#[test]
fn test_to_vec_with_basic_serializable_struct() {
    use serde::Serialize;
    
    #[derive(Serialize)]
    struct BasicStruct {
        field1: i32,
        field2: String,
    }
    
    let value = BasicStruct {
        field1: 42,
        field2: String::from("Hello"),
    };
    
    let result = to_vec(&value);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), br#"{"field1":42,"field2":"Hello"}"#);
}

#[test]
fn test_to_vec_with_empty_struct() {
    use serde::Serialize;
    
    #[derive(Serialize)]
    struct EmptyStruct;
    
    let value = EmptyStruct;
    
    let result = to_vec(&value);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), br#"{}"#);
}

#[test]
fn test_to_vec_with_serializable_vec() {
    use serde::Serialize;

    #[derive(Serialize)]
    struct VecStruct {
        items: Vec<i32>,
    }
    
    let value = VecStruct {
        items: vec![1, 2, 3, 4],
    };
    
    let result = to_vec(&value);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), br#"{"items":[1,2,3,4]}"#);
}

#[test]
fn test_to_vec_with_nested_serializable_structs() {
    use serde::Serialize;

    #[derive(Serialize)]
    struct InnerStruct {
        inner_field: String,
    }

    #[derive(Serialize)]
    struct OuterStruct {
        outer_field: InnerStruct,
    }
    
    let value = OuterStruct {
        outer_field: InnerStruct {
            inner_field: String::from("World"),
        },
    };
    
    let result = to_vec(&value);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), br#"{"outer_field":{"inner_field":"World"}}"#);
}

#[should_panic]
fn test_to_vec_with_non_serializable_struct() {
    use serde::ser::Serialize;

    struct NonSerializableStruct;

    let value = NonSerializableStruct;
    
    let _result = to_vec(&value);
}

