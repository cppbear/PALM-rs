// Answer 0

#[test]
fn test_range_to_visitor_expecting() {
    use std::fmt;
    use std::marker::PhantomData;

    struct TestFormatter;
    
    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            assert_eq!(s, "test");
            Ok(())
        }
    }

    struct TestVisitor {
        expecting: &'static str,
        phantom: PhantomData<()>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.expecting)
        }
    }

    let visitor = TestVisitor {
        expecting: "test",
        phantom: PhantomData,
    };

    let mut formatter = TestFormatter;
    let result = visitor.expecting(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_range_to_visitor_expecting_empty_string() {
    use std::fmt;
    use std::marker::PhantomData;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            assert_eq!(s, "");
            Ok(())
        }
    }

    struct TestVisitor {
        expecting: &'static str,
        phantom: PhantomData<()>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.expecting)
        }
    }

    let visitor = TestVisitor {
        expecting: "",
        phantom: PhantomData,
    };

    let mut formatter = TestFormatter;
    let result = visitor.expecting(&mut formatter);
    assert!(result.is_ok());
}

