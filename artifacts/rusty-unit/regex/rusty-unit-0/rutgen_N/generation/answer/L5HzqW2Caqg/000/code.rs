// Answer 0

#[test]
fn test_fmt_display() {
    struct TestStruct {
        regex: String,
    }

    impl std::fmt::Display for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.regex)
        }
    }

    let test_instance = TestStruct {
        regex: String::from("^[a-zA-Z]+$"),
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", test_instance);
    
    assert!(result.is_ok());
    assert_eq!(output, "^[a-zA-Z]+$");
}

