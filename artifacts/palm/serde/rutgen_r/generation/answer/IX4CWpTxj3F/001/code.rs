// Answer 0

#[test]
fn test_fmt_with_other_variant() {
    use std::fmt;

    struct Unexpected {
        value: String,
    }

    impl fmt::Display for Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "{}", self.value)
        }
    }

    struct TestStruct;

    impl TestStruct {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            let other = Unexpected { value: String::from("custom unexpected value") };
            write!(formatter, "{}", other)
        }
    }

    let test_instance = TestStruct;
    let mut buffer = String::new();
    let result = test_instance.fmt(&mut fmt::Formatter::new(&mut buffer));
    
    assert!(result.is_ok());
    assert_eq!(buffer, "custom unexpected value");
}

