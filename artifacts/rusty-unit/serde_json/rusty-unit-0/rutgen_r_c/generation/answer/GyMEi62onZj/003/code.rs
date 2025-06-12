// Answer 0

#[test]
fn test_fmt_value_array_with_error() {
    struct MockFormatter {
        should_fail: bool,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_fail {
                Err(fmt::Error)
            } else {
                Ok(())
            }
        }
    }

    let mock_formatter = &mut MockFormatter { should_fail: true };
    
    let value = Value::Array(vec![
        Value::String(String::from("item1")),
        Value::String(String::from("item2")),
    ]);

    let result = value.fmt(mock_formatter);
    
    assert!(result.is_err());
}

#[test]
fn test_fmt_value_array_without_error() {
    struct MockFormatter {
        should_fail: bool,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_fail {
                Err(fmt::Error)
            } else {
                Ok(())
            }
        }
    }

    let mock_formatter = &mut MockFormatter { should_fail: false };
    
    let value = Value::Array(vec![
        Value::String(String::from("item1")),
        Value::String(String::from("item2")),
    ]);

    let result = value.fmt(mock_formatter);
    
    assert!(result.is_ok());
}

