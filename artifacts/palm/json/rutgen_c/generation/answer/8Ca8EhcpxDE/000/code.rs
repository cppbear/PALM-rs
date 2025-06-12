// Answer 0

#[test]
fn test_serialize_struct_variant_end() {
    struct TestSerializeStructVariant {
        name: String,
        map: Map<String, Value>,
    }

    impl serde::ser::SerializeStructVariant for TestSerializeStructVariant {
        type Ok = Value;
        type Error = Error;

        fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Value> {
            let mut object = Map::new();
            object.insert(self.name, Value::Object(self.map));
            Ok(Value::Object(object))
        }
    }

    let mut map = Map::new();
    map.insert("key".to_string(), Value::String("value".to_string()));
    
    let variant = TestSerializeStructVariant {
        name: "test_variant".to_string(),
        map,
    };
    
    let result = variant.end().unwrap();
    
    if let Value::Object(obj) = result {
        assert!(obj.len() == 1);
        assert!(obj.get("test_variant").is_some());
    } else {
        panic!("Expected Value::Object but got a different variant.");
    }
}

