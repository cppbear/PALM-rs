// Answer 0

#[test]
fn test_index_or_insert_valid_index() {
    struct IndexImpl(usize);
    impl private::Sealed for IndexImpl {}
    impl Index for IndexImpl {
        fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
            None
        }
        fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
            None
        }
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

    let mut json_array = Value::Array(vec![
        Value::Number(42.into()),
        Value::Number(100.into()),
    ]);
    let index = IndexImpl(1);
    assert_eq!(index.index_or_insert(&mut json_array), &mut Value::Number(100.into()));
}

#[test]
#[should_panic(expected = "cannot access index 2 of JSON array of length 2")]
fn test_index_or_insert_out_of_bounds_index() {
    struct IndexImpl(usize);
    impl private::Sealed for IndexImpl {}
    impl Index for IndexImpl {
        fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
            None
        }
        fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
            None
        }
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

    let mut json_array = Value::Array(vec![
        Value::Number(42.into()),
        Value::Number(100.into()),
    ]);
    let index = IndexImpl(2);
    index.index_or_insert(&mut json_array);
}

#[test]
#[should_panic(expected = "cannot access index 0 of JSON")]
fn test_index_or_insert_non_array_json() {
    struct IndexImpl(usize);
    impl private::Sealed for IndexImpl {}
    impl Index for IndexImpl {
        fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
            None
        }
        fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
            None
        }
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

    let mut json_object = Value::Object(Map::new());
    let index = IndexImpl(0);
    index.index_or_insert(&mut json_object);
}

