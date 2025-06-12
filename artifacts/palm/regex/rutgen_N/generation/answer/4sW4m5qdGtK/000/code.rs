// Answer 0

#[test]
fn test_parse_many_with_empty_patterns() {
    struct TestStruct {
        arg_patterns: Vec<String>,
    }

    impl TestStruct {
        fn parse_many(&self) -> Result<Vec<String>, &'static str> {
            self.arg_patterns.iter().map(|s| Ok(s.clone())).collect()
        }
    }

    let test_instance = TestStruct {
        arg_patterns: vec![],
    };

    let result = test_instance.parse_many();
    assert_eq!(result.unwrap(), vec![]);
}

#[test]
fn test_parse_many_with_single_pattern() {
    struct TestStruct {
        arg_patterns: Vec<String>,
    }

    impl TestStruct {
        fn parse_many(&self) -> Result<Vec<String>, &'static str> {
            self.arg_patterns.iter().map(|s| Ok(s.clone())).collect()
        }
    }

    let test_instance = TestStruct {
        arg_patterns: vec![String::from("pattern1")],
    };

    let result = test_instance.parse_many();
    assert_eq!(result.unwrap(), vec![String::from("pattern1")]);
}

#[test]
fn test_parse_many_with_multiple_patterns() {
    struct TestStruct {
        arg_patterns: Vec<String>,
    }

    impl TestStruct {
        fn parse_many(&self) -> Result<Vec<String>, &'static str> {
            self.arg_patterns.iter().map(|s| Ok(s.clone())).collect()
        }
    }

    let test_instance = TestStruct {
        arg_patterns: vec![String::from("pattern1"), String::from("pattern2")],
    };

    let result = test_instance.parse_many();
    assert_eq!(result.unwrap(), vec![String::from("pattern1"), String::from("pattern2")]);
}

