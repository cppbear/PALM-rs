// Answer 0

#[test]
fn test_expecting() {
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
    let visitor = TestVisitor {
        expecting: "a test value",
    };

    visitor.expecting(&mut formatter).unwrap();
    assert_eq!(buf, "a test value");
}

#[test]
fn test_expecting_empty_string() {
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
    let visitor = TestVisitor {
        expecting: "",
    };

    visitor.expecting(&mut formatter).unwrap();
    assert_eq!(buf, "");
}

