// Answer 0

#[test]
fn test_invalid_type_null() {
    use serde::de::Error;
    use serde::de::Unexpected;

    struct ExpectedType;

    impl Expected for ExpectedType {
        fn type_name(&self) -> &str {
            "ExpectedType"
        }
    }

    let value = Value::Null;
    let result: Result<_, _> = std::panic::catch_unwind(|| {
        value.invalid_type::<ExpectedType>(&ExpectedType);
    });

    assert!(result.is_err());
}

#[test]
fn test_invalid_type_bool() {
    use serde::de::Error;
    use serde::de::Unexpected;

    struct ExpectedType;

    impl Expected for ExpectedType {
        fn type_name(&self) -> &str {
            "ExpectedType"
        }
    }

    let value = Value::Bool(true);
    let result: Result<_, _> = std::panic::catch_unwind(|| {
        value.invalid_type::<ExpectedType>(&ExpectedType);
    });

    assert!(result.is_err());
}

#[test]
fn test_invalid_type_number() {
    use serde::de::Error;
    use serde::de::Unexpected;

    struct ExpectedType;

    impl Expected for ExpectedType {
        fn type_name(&self) -> &str {
            "ExpectedType"
        }
    }

    let number_data = Number { n: 42 }; // Assuming some value for N
    let value = Value::Number(number_data);
    let result: Result<_, _> = std::panic::catch_unwind(|| {
        value.invalid_type::<ExpectedType>(&ExpectedType);
    });

    assert!(result.is_err());
}

#[test]
fn test_invalid_type_string() {
    use serde::de::Error;
    use serde::de::Unexpected;

    struct ExpectedType;

    impl Expected for ExpectedType {
        fn type_name(&self) -> &str {
            "ExpectedType"
        }
    }

    let value = Value::String(String::from("testing"));
    let result: Result<_, _> = std::panic::catch_unwind(|| {
        value.invalid_type::<ExpectedType>(&ExpectedType);
    });

    assert!(result.is_err());
}

#[test]
fn test_invalid_type_array() {
    use serde::de::Error;
    use serde::de::Unexpected;

    struct ExpectedType;

    impl Expected for ExpectedType {
        fn type_name(&self) -> &str {
            "ExpectedType"
        }
    }

    let value = Value::Array(vec![Value::Null, Value::Bool(false)]);
    let result: Result<_, _> = std::panic::catch_unwind(|| {
        value.invalid_type::<ExpectedType>(&ExpectedType);
    });

    assert!(result.is_err());
}

#[test]
fn test_invalid_type_object() {
    use serde::de::Error;
    use serde::de::Unexpected;

    struct ExpectedType;

    impl Expected for ExpectedType {
        fn type_name(&self) -> &str {
            "ExpectedType"
        }
    }

    let mut map = Map { map: alloc::collections::BTreeMap::new() };
    map.map.insert(String::from("key"), Value::Null);
    let value = Value::Object(map);
    let result: Result<_, _> = std::panic::catch_unwind(|| {
        value.invalid_type::<ExpectedType>(&ExpectedType);
    });

    assert!(result.is_err());
}

