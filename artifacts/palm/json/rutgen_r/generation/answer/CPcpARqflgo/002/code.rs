// Answer 0

#[test]
fn test_index_or_insert_valid_index() {
    struct IndexWrapper(usize);
    
    impl IndexWrapper {
        fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
            match v {
                Value::Array(vec) => {
                    let len = vec.len();
                    vec.get_mut(self.0).unwrap_or_else(|| {
                        panic!(
                            "cannot access index {} of JSON array of length {}",
                            self.0, len
                        )
                    })
                }
                _ => panic!("cannot access index {} of JSON {}", self.0, Type(v)),
            }
        }
    }

    let mut value = Value::Array(vec![Value::Number(1.into()), Value::Number(2.into())]);
    let index = IndexWrapper(1);
    let result = index.index_or_insert(&mut value);
    assert_eq!(result, &mut Value::Number(2.into()));
}

#[test]
#[should_panic(expected = "cannot access index 2 of JSON array of length 2")]
fn test_index_or_insert_out_of_bounds() {
    struct IndexWrapper(usize);
    
    impl IndexWrapper {
        fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
            match v {
                Value::Array(vec) => {
                    let len = vec.len();
                    vec.get_mut(self.0).unwrap_or_else(|| {
                        panic!(
                            "cannot access index {} of JSON array of length {}",
                            self.0, len
                        )
                    })
                }
                _ => panic!("cannot access index {} of JSON {}", self.0, Type(v)),
            }
        }
    }

    let mut value = Value::Array(vec![Value::Number(1.into()), Value::Number(2.into())]);
    let index = IndexWrapper(2);
    index.index_or_insert(&mut value);
}

#[test]
#[should_panic(expected = "cannot access index 0 of JSON")]
fn test_index_or_insert_non_array() {
    struct IndexWrapper(usize);
    
    impl IndexWrapper {
        fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
            match v {
                Value::Array(vec) => {
                    let len = vec.len();
                    vec.get_mut(self.0).unwrap_or_else(|| {
                        panic!(
                            "cannot access index {} of JSON array of length {}",
                            self.0, len
                        )
                    })
                }
                _ => panic!("cannot access index {} of JSON {}", self.0, Type(v)),
            }
        }
    }

    let mut value = Value::Object(serde_json::map::Map::new());
    let index = IndexWrapper(0);
    index.index_or_insert(&mut value);
}

