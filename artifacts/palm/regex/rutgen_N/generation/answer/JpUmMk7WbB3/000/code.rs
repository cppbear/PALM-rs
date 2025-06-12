// Answer 0

#[test]
fn test_fmt() {
    struct TestRegex {
        s: String,
    }
    
    impl TestRegex {
        fn as_str(&self) -> &str {
            &self.s
        }
    }
    
    impl std::fmt::Display for TestRegex {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.as_str())
        }
    }
    
    let regex = TestRegex { s: String::from("a+b*") };
    let mut output = String::new();
    let result = regex.fmt(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "a+b*");
}

