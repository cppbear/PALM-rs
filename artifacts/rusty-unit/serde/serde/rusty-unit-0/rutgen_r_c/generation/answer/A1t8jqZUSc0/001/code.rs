// Answer 0

#[test]
fn test_fmt_with_valid_formatter() {
    struct TestExpected;

    impl Expected for TestExpected {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("TestExpected")
        }
    }

    let expected = TestExpected;
    let mut output = String::new();
    let mut formatter = std::fmt::Formatter::new(&mut output);

    assert!(expected.fmt(&mut formatter).is_ok());
    assert_eq!(output, "TestExpected");
}

#[test]
#[should_panic]
fn test_fmt_with_panic_condition() {
    struct PanicExpected;

    impl Expected for PanicExpected {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            panic!("Intentional Panic")
        }
    }

    let expected = PanicExpected;
    let mut formatter = std::fmt::Formatter::new(&mut String::new());

    expected.fmt(&mut formatter); // This should panic
}

