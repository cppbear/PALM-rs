// Answer 0

#[test]
fn test_fmt_null() {
    struct TestValue;

    impl fmt::Debug for TestValue {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("Null")
        }
    }

    let value = Value::Null;
    let mut output = String::new();
    let result = write!(&mut output, "{}", value);
    assert!(result.is_ok());
    assert_eq!(output, "Null");
}

#[test]
fn test_fmt_bool_true() {
    struct TestValue;

    impl fmt::Debug for TestValue {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("Bool(true)")
        }
    }

    let value = Value::Bool(true);
    let mut output = String::new();
    let result = write!(&mut output, "{}", value);
    assert!(result.is_ok());
    assert_eq!(output, "Bool(true)");
}

#[test]
fn test_fmt_bool_false() {
    struct TestValue;

    impl fmt::Debug for TestValue {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("Bool(false)")
        }
    }

    let value = Value::Bool(false);
    let mut output = String::new();
    let result = write!(&mut output, "{}", value);
    assert!(result.is_ok());
    assert_eq!(output, "Bool(false)");
}

