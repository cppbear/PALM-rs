// Answer 0

#[test]
fn test_serialize_str() {
    struct TestSerializer {
        data: String,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                data: String::new(),
            }
        }
        
        fn serialize_str(&mut self, value: &str) -> Result<(), String> {
            self.data.push_str(value);
            Ok(())
        }
    }
    
    let mut serializer = TestSerializer::new();
    let result = serializer.serialize_str("test");
    assert!(result.is_ok());
    assert_eq!(serializer.data, "test");
}

#[test]
fn test_serialize_str_empty() {
    struct TestSerializer {
        data: String,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                data: String::new(),
            }
        }
        
        fn serialize_str(&mut self, value: &str) -> Result<(), String> {
            self.data.push_str(value);
            Ok(())
        }
    }
    
    let mut serializer = TestSerializer::new();
    let result = serializer.serialize_str("");
    assert!(result.is_ok());
    assert_eq!(serializer.data, "");
}

