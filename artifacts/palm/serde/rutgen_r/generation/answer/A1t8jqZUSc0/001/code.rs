// Answer 0

#[test]
fn test_fmt_valid_case() {
    struct TestFormatter;

    impl std::fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> std::fmt::Result {
            Ok(())
        }
    }

    struct TestStruct;

    impl TestStruct {
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("test")
        }

        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            self.expecting(formatter)
        }
    }

    let test_instance = TestStruct;
    let mut test_formatter = TestFormatter;

    // This should not panic and should complete successfully
    let result = test_instance.fmt(&mut test_formatter);
    assert!(result.is_ok());
}

#[should_panic]
#[test]
fn test_fmt_expectation_panic() {
    struct PanicFormatter;

    impl std::fmt::Write for PanicFormatter {
        fn write_str(&mut self, _: &str) -> std::fmt::Result {
            panic!("Funny panic!")
        }
    }

    struct PanicStruct;

    impl PanicStruct {
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("This will cause a panic")
        }

        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            self.expecting(formatter)
        }
    }

    let panic_instance = PanicStruct;
    let mut panic_formatter = PanicFormatter;

    // This will panic when fmt is called
    panic_instance.fmt(&mut panic_formatter);
}

