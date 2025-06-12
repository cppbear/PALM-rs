// Answer 0

#[test]
fn test_parse_one_valid_pattern() {
    struct TestStruct {
        arg_pattern: String,
    }

    impl TestStruct {
        fn new(arg_pattern: &str) -> Self {
            TestStruct {
                arg_pattern: arg_pattern.to_string(),
            }
        }
        
        fn parse_one(&self) -> Result<Hir, &'static str> {
            parse(&self.arg_pattern)
        }
    }

    let test_instance = TestStruct::new("a*b");
    let result = test_instance.parse_one();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_parse_one_invalid_pattern() {
    struct TestStruct {
        arg_pattern: String,
    }

    impl TestStruct {
        fn new(arg_pattern: &str) -> Self {
            TestStruct {
                arg_pattern: arg_pattern.to_string(),
            }
        }
        
        fn parse_one(&self) -> Result<Hir, &'static str> {
            parse(&self.arg_pattern)
        }
    }

    let test_instance = TestStruct::new("*invalid");
    let _ = test_instance.parse_one().unwrap(); // This should panic
}

