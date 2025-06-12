// Answer 0

#[test]
fn test_expectation_tag () {
    struct TestVisitor {
        name: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            write!(fmt, "a type tag `{}` or any other value", self.name)
        }
    }

    let mut output = String::new();
    let visitor = TestVisitor { name: "test_tag" };
    let result = visitor.expecting(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "a type tag `test_tag` or any other value");
}

#[test]
fn test_expectation_empty_tag() {
    struct TestVisitor {
        name: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            write!(fmt, "a type tag `{}` or any other value", self.name)
        }
    }

    let mut output = String::new();
    let visitor = TestVisitor { name: "" };
    let result = visitor.expecting(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "a type tag `` or any other value");
}

#[test]
fn test_expectation_long_tag() {
    struct TestVisitor {
        name: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            write!(fmt, "a type tag `{}` or any other value", self.name)
        }
    }

    let mut output = String::new();
    let visitor = TestVisitor { name: "this_is_a_very_long_tag_that_should_still_work" };
    let result = visitor.expecting(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "a type tag `this_is_a_very_long_tag_that_should_still_work` or any other value");
}

