// Answer 0

#[test]
fn test_regex_display() {
    struct TestExec {
        regex_strings: Vec<String>,
    }
    
    impl TestExec {
        fn new(regex_strings: Vec<&str>) -> Self {
            TestExec {
                regex_strings: regex_strings.iter().map(|&s| s.to_string()).collect(),
            }
        }
        
        fn regex_strings(&self) -> &Vec<String> {
            &self.regex_strings
        }
    }

    impl fmt::Display for TestExec {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.regex_strings()[0])
        }
    }

    let exec = TestExec::new(vec!["a.*b"]);
    let regex = Regex(exec);

    let mut output = String::new();
    let result = write!(&mut output, "{}", regex);

    assert!(result.is_ok());
    assert_eq!(output, "a.*b");
}

