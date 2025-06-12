// Answer 0

#[test]
fn test_fmt_display() {
    use std::fmt;

    struct TestStruct;

    impl fmt::Display for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestStruct")
        }
    }

    let test_instance = TestStruct;
    let mut buffer = String::new();
    let result = fmt::write(&mut buffer, |f| test_instance.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(buffer, "TestStruct");
}

