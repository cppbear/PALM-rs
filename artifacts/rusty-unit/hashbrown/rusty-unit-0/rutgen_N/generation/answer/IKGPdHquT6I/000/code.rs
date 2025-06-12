// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    #[derive(Clone)]
    struct TestStruct {
        entries: Vec<i32>,
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.entries.clone()).finish()
        }
    }

    let test_instance = TestStruct {
        entries: vec![1, 2, 3],
    };
    
    let mut output = String::new();
    let result = write!(output, "{:?}", test_instance);
    
    assert!(result.is_ok());
    assert_eq!(output, "[1, 2, 3]");
}

