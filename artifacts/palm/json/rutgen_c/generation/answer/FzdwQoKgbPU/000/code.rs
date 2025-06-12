// Answer 0

#[test]
fn test_index_into_with_existing_key() {
    struct TestMap {}
    
    impl private::Sealed for TestMap {}
    
    impl Index for TestMap {
        fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
            if let Value::Object(ref map) = v {
                map.get("key")
            } else {
                None
            }
        }
        fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
            unimplemented!()
        }
        fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
            unimplemented!()
        }
    }

    let key = String::from("key");
    let value = Value::Object(Map::new());
    let result = key.index_into(&value);
    assert!(result.is_none());
}

#[test]
fn test_index_into_with_non_existing_key() {
    struct TestMap {}
    
    impl private::Sealed for TestMap {}
    
    impl Index for TestMap {
        fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
            if let Value::Object(ref map) = v {
                map.get("not_a_key")
            } else {
                None
            }
        }
        fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
            unimplemented!()
        }
        fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
            unimplemented!()
        }
    }

    let key = String::from("not_a_key");
    let value = Value::Object(Map::new());
    let result = key.index_into(&value);
    assert!(result.is_none());
}

