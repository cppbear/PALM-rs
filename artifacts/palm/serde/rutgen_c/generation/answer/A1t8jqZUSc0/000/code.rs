// Answer 0

#[test]
fn test_fmt_with_valid_formatter() {
    struct TestExpected;

    impl Expected for TestExpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("TestExpected")
        }
    }

    let expected = TestExpected;
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);

    expected.fmt(formatter).unwrap();
    assert_eq!(buffer, "TestExpected");
}

#[test]
fn test_fmt_with_empty_formatter() {
    struct EmptyExpected;

    impl Expected for EmptyExpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("")
        }
    }

    let expected = EmptyExpected;
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);

    expected.fmt(formatter).unwrap();
    assert_eq!(buffer, "");
}

