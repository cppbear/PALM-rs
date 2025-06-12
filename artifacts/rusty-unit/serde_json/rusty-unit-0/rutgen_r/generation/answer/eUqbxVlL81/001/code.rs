// Answer 0

#[test]
fn test_serialize_unit_variant_valid() {
    struct DummySerializer {
        output: String,
    }

    impl DummySerializer {
        fn new() -> Self {
            DummySerializer {
                output: String::new(),
            }
        }
        
        fn serialize_str(&mut self, value: &'static str) -> Result<(), String> {
            self.output.push_str(value);
            Ok(())
        }
    }
  
    let mut serializer = DummySerializer::new();
    let result = serialize_unit_variant(&mut serializer, "TestName", 0, "TestVariant");
    assert!(result.is_ok());
    assert_eq!(serializer.output, "TestVariant");
}

#[test]
#[should_panic]
fn test_serialize_unit_variant_empty_variant() {
    struct DummySerializer {
        output: String,
    }

    impl DummySerializer {
        fn new() -> Self {
            DummySerializer {
                output: String::new(),
            }
        }
        
        fn serialize_str(&mut self, value: &'static str) -> Result<(), String> {
            if value.is_empty() {
                panic!("Empty variant string cannot be serialized");
            }
            self.output.push_str(value);
            Ok(())
        }
    }
  
    let mut serializer = DummySerializer::new();
    let _ = serialize_unit_variant(&mut serializer, "TestName", 1, "");
}

