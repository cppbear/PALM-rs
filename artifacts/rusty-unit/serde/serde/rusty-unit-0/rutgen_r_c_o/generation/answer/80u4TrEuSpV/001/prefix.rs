// Answer 0

#[test]
fn test_expectation_string_1() {
    struct TestVisitor {
        expecting: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.expecting)
        }
    }

    let visitor = TestVisitor { expecting: "Test expecting 1" };
    let mut formatter = std::fmt::Formatter::new();
    let _ = visitor.expecting(&mut formatter);
}

#[test]
fn test_expectation_string_2() {
    struct TestVisitor {
        expecting: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.expecting)
        }
    }

    let visitor = TestVisitor { expecting: "Test expecting 2" };
    let mut formatter = std::fmt::Formatter::new();
    let _ = visitor.expecting(&mut formatter);
}

#[test]
fn test_expectation_string_edge_case() {
    struct TestVisitor {
        expecting: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.expecting)
        }
    }

    let visitor = TestVisitor { expecting: "" }; // Edge case: empty string
    let mut formatter = std::fmt::Formatter::new();
    let _ = visitor.expecting(&mut formatter);
}

