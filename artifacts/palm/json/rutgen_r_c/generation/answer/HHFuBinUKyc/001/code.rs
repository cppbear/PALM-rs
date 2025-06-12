// Answer 0

#[test]
fn test_index_on_object_with_existing_key() {
    struct TestIndex;

    impl Index for TestIndex {
        fn index_into(&self, value: &Value) -> Option<&Value> {
            if let Value::Object(ref map) = value {
                Some(&map.map["x"])
            } else {
                None
            }
        }
    }

    let data = Value::Object(Map { map: vec![("x".to_owned(), Value::Array(vec![Value::String("z".to_owned()), Value::String("zz".to_owned())]))].into_iter().collect() });

    assert_eq!(data[TestIndex], Value::Array(vec![Value::String("z".to_owned()), Value::String("zz".to_owned())]));
}

#[test]
fn test_index_on_object_with_non_existing_key() {
    struct TestIndex;

    impl Index for TestIndex {
        fn index_into(&self, value: &Value) -> Option<&Value> {
            if let Value::Object(ref map) = value {
                map.map.get("non_existing_key")
            } else {
                None
            }
        }
    }

    let data = Value::Object(Map { map: vec![("x".to_owned(), Value::Array(vec![Value::String("z".to_owned())]))].into_iter().collect() });

    assert_eq!(data[TestIndex], Value::Null);
}

#[test]
fn test_index_on_array_with_valid_index() {
    struct TestIndex(usize);

    impl Index for TestIndex {
        fn index_into(&self, value: &Value) -> Option<&Value> {
            if let Value::Array(ref array) = value {
                if self.0 < array.len() {
                    Some(&array[self.0])
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    let data = Value::Array(vec![Value::String("z".to_owned()), Value::String("zz".to_owned())]);

    assert_eq!(data[TestIndex(0)], Value::String("z".to_owned()));
}

#[test]
fn test_index_on_array_with_invalid_index() {
    struct TestIndex(usize);

    impl Index for TestIndex {
        fn index_into(&self, value: &Value) -> Option<&Value> {
            if let Value::Array(ref array) = value {
                if self.0 < array.len() {
                    Some(&array[self.0])
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    let data = Value::Array(vec![Value::String("z".to_owned())]);

    assert_eq!(data[TestIndex(1)], Value::Null);
}

#[test]
fn test_index_on_non_object_type() {
    struct TestIndex;

    impl Index for TestIndex {
        fn index_into(&self, value: &Value) -> Option<&Value> {
            if let Value::Number(_) = value {
                Some(&Value::Null) // Simply returning Null for the sake of this test case
            } else {
                None
            }
        }
    }

    let data = Value::Number(Number { n: 42 });

    assert_eq!(data[TestIndex], Value::Null);
}

