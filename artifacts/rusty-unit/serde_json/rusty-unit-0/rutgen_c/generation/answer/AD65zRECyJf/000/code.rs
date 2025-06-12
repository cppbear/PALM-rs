// Answer 0

#[test]
fn test_get_from_object() {
    struct IndexImpl<'a> {
        key: &'a str,
    }
    
    impl<'a> Index for IndexImpl<'a> {
        fn index_into(&self, value: &Value) -> Option<&Value> {
            match value {
                Value::Object(map) => map.map.get(self.key),
                _ => None,
            }
        }
    }

    let object = Value::Object(Map { map: vec![("A".to_string(), Value::Number(Number { n: 65 }))].into_iter().collect() });
    assert_eq!(*object.get(IndexImpl { key: "A" }).unwrap(), Value::Number(Number { n: 65 }));
}

#[test]
fn test_get_from_array() {
    struct IndexImpl {
        index: usize,
    }
    
    impl Index for IndexImpl {
        fn index_into(&self, value: &Value) -> Option<&Value> {
            match value {
                Value::Array(array) => array.get(self.index),
                _ => None,
            }
        }
    }

    let array = Value::Array(vec![Value::String("A".to_string()), Value::String("B".to_string()), Value::String("C".to_string())]);
    assert_eq!(*array.get(IndexImpl { index: 2 }).unwrap(), Value::String("C".to_string()));
}

#[test]
fn test_get_key_not_in_object() {
    struct IndexImpl<'a> {
        key: &'a str,
    }
    
    impl<'a> Index for IndexImpl<'a> {
        fn index_into(&self, value: &Value) -> Option<&Value> {
            match value {
                Value::Object(map) => map.map.get(self.key),
                _ => None,
            }
        }
    }

    let object = Value::Object(Map { map: vec![("A".to_string(), Value::Number(Number { n: 65 }))].into_iter().collect() });
    assert_eq!(object.get(IndexImpl { key: "D" }), None);
}

#[test]
fn test_get_out_of_bounds_in_array() {
    struct IndexImpl {
        index: usize,
    }
    
    impl Index for IndexImpl {
        fn index_into(&self, value: &Value) -> Option<&Value> {
            match value {
                Value::Array(array) => array.get(self.index),
                _ => None,
            }
        }
    }

    let array = Value::Array(vec![Value::String("A".to_string()), Value::String("B".to_string())]);
    assert_eq!(array.get(IndexImpl { index: 5 }), None);
}

