// Answer 0

#[test]
fn test_regex_debug_format() {
    struct TestExec;

    impl fmt::Display for TestExec {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestExec")
        }
    }

    let exec = TestExec;
    let regex = Regex(exec);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", regex);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "TestExec");
}

#[test]
fn test_empty_regex_debug_format() {
    struct EmptyExec;

    impl fmt::Display for EmptyExec {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "")
        }
    }

    let exec = EmptyExec;
    let regex = Regex(exec);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", regex);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "");
}

#[test]
#[should_panic]
fn test_regex_debug_format_panic() {
    struct PanickingExec;

    impl fmt::Display for PanickingExec {
        fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
            panic!("This is a panic from fmt");
        }
    }

    let exec = PanickingExec;
    let regex = Regex(exec);
    let _ = format!("{:?}", regex);
}

