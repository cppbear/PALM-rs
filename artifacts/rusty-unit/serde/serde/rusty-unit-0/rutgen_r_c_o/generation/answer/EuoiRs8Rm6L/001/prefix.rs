// Answer 0

#[test]
fn test_expectation_with_valid_formatter() {
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

    let mut buf = String::new();
    let mut formatter = fmt::Formatter::new(&mut buf);
    let visitor = TestVisitor { expecting: "Test expecting" };
    let _ = visitor.expecting(&mut formatter);
}

#[test]
fn test_expectation_with_empty_string() {
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

    let mut buf = String::new();
    let mut formatter = fmt::Formatter::new(&mut buf);
    let visitor = TestVisitor { expecting: "" };
    let _ = visitor.expecting(&mut formatter);
}

#[test]
fn test_expectation_with_long_string() {
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

    let long_string = "a".repeat(10000);
    let mut buf = String::new();
    let mut formatter = fmt::Formatter::new(&mut buf);
    let visitor = TestVisitor { expecting: &long_string };
    let _ = visitor.expecting(&mut formatter);
}

