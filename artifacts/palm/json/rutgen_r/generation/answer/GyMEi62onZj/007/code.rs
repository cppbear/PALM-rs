// Answer 0

#[test]
fn test_value_fmt_null() {
    struct MockFormatter;
    
    impl std::fmt::Write for MockFormatter {
        fn write_str(&mut self, _: &str) -> std::fmt::Result {
            Ok(())
        }
    }

    let value = Value::Null;
    let mut formatter = MockFormatter;
    assert!(value.fmt(&mut formatter).is_ok());
}

#[test]
fn test_value_fmt_bool_true() {
    struct MockFormatter;
    
    impl std::fmt::Write for MockFormatter {
        fn write_str(&mut self, _: &str) -> std::fmt::Result {
            Ok(())
        }
    }

    let value = Value::Bool(true);
    let mut formatter = MockFormatter;
    assert!(value.fmt(&mut formatter).is_ok());
}

#[test]
fn test_value_fmt_bool_false() {
    struct MockFormatter;
    
    impl std::fmt::Write for MockFormatter {
        fn write_str(&mut self, _: &str) -> std::fmt::Result {
            Ok(())
        }
    }

    let value = Value::Bool(false);
    let mut formatter = MockFormatter;
    assert!(value.fmt(&mut formatter).is_ok());
}

