// Answer 0

#[test]
fn test_expectation_with_valid_formatter() {
    use std::fmt;

    struct TestVisitor {
        expecting: &'static str,
    }

    impl Visitor<'static> for TestVisitor {
        type Value = ();
        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_str(self.expecting)
        }
    }

    let visitor = TestVisitor {
        expecting: "Test Expectation",
    };

    let mut output = String::new();
    let result = visitor.expecting(&mut fmt::Formatter::new(&mut output));

    assert!(result.is_ok());
    assert_eq!(output, "Test Expectation");
}

#[test]
#[should_panic]
fn test_expectation_panic_on_invalid_formatter() {
    use std::fmt;

    struct FailingVisitor {
        expecting: &'static str,
    }

    impl Visitor<'static> for FailingVisitor {
        type Value = ();
        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            // This will panic if fmt.write_str causes a panic
            fmt.write_str(self.expecting)
        }
    }

    let failing_visitor = FailingVisitor {
        expecting: "Expecting Panic",
    };

    // Creating a formatter that will panic
    struct PanicFormatter;

    impl fmt::Write for PanicFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            panic!("Formatter panic")
        }
    }

    let mut panic_formatter = PanicFormatter;
    let _ = failing_visitor.expecting(&mut panic_formatter);
}

