// Answer 0

#[test]
fn test_get_mut_from_object() {
    struct TestIndex;
    
    impl Index for TestIndex {
        fn index_into_mut(self, value: &mut Value) -> Option<&mut Value> {
            if let Value::Object(ref mut map) = value {
                map.map.get_mut("A")
            } else {
                None
            }
        }
    }

    let mut object = Value::Object(Map {
        map: {
            let mut temp_map = std::collections::BTreeMap::new();
            temp_map.insert(String::from("A"), Value::Number(Number { n: 65 }));
            temp_map.insert(String::from("B"), Value::Number(Number { n: 66 }));
            temp_map.insert(String::from("C"), Value::Number(Number { n: 67 }));
            temp_map
        }
    });
    
    *object.get_mut(TestIndex).unwrap() = Value::Number(Number { n: 69 });
    
    if let Value::Object(ref map) = object {
        assert_eq!(map.map.get("A"), Some(&Value::Number(Number { n: 69 })));
    }
}

#[test]
fn test_get_mut_from_array() {
    struct TestIndex(usize);
    
    impl Index for TestIndex {
        fn index_into_mut(self, value: &mut Value) -> Option<&mut Value> {
            if let Value::Array(ref mut array) = value {
                if self.0 < array.len() {
                    Some(&mut array[self.0])
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    let mut array = Value::Array(vec![
        Value::String(String::from("A")),
        Value::String(String::from("B")),
        Value::String(String::from("C")),
    ]);
    
    *array.get_mut(TestIndex(2)).unwrap() = Value::String(String::from("D"));
    
    if let Value::Array(ref arr) = array {
        assert_eq!(arr.get(2), Some(&Value::String(String::from("D"))));
    }
}

#[test]
fn test_get_mut_fail_case() {
    struct TestIndex;
    
    impl Index for TestIndex {
        fn index_into_mut(self, value: &mut Value) -> Option<&mut Value> {
            if let Value::Array(_) = value {
                None
            } else {
                None
            }
        }
    }

    let mut object = Value::Object(Map {
        map: {
            let mut temp_map = std::collections::BTreeMap::new();
            temp_map.insert(String::from("A"), Value::Number(Number { n: 65 }));
            temp_map
        }
    });
    
    let result = object.get_mut(TestIndex);
    assert!(result.is_none());
}

#[test]
fn test_get_mut_out_of_bounds() {
    struct TestIndex(usize);
    
    impl Index for TestIndex {
        fn index_into_mut(self, value: &mut Value) -> Option<&mut Value> {
            if let Value::Array(ref mut array) = value {
                if self.0 < array.len() {
                    Some(&mut array[self.0])
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    let mut array = Value::Array(vec![
        Value::String(String::from("A")),
        Value::String(String::from("B")),
    ]);
    
    let result = array.get_mut(TestIndex(2)); 
    assert!(result.is_none());
}

