// Answer 0

#[test]
fn test_fmt() {
    use std::fmt::{self, Write};

    struct TestStruct;

    impl fmt::Display for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestStruct instance")
        }
    }

    let test_instance = TestStruct;
    let mut buffer = String::new();
    let result = test_instance.fmt(&mut fmt::Formatter::new(&mut buffer));
    
    assert!(result.is_ok());
    assert_eq!(buffer, "TestStruct instance");
}

