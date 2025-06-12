// Answer 0

#[test]
fn test_expecting() {
    use std::fmt;

    struct TestVisitor {
        expecting: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.expecting)
        }
    }

    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    let visitor = TestVisitor { expecting: "Test Expectation" };

    visitor.expecting(formatter).unwrap();

    assert_eq!(output, "Test Expectation");
}

#[test]
fn test_expecting_empty() {
    use std::fmt;

    struct EmptyVisitor {
        expecting: &'static str,
    }

    impl<'de> Visitor<'de> for EmptyVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.expecting)
        }
    }

    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    let visitor = EmptyVisitor { expecting: "" };

    visitor.expecting(formatter).unwrap();

    assert_eq!(output, "");
}

