// Answer 0

#[test]
fn test_visit_bool() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = IgnoredAny;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Test visitor expecting")
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_bool::<()>(true);
    assert_eq!(result, Ok(IgnoredAny));
}

#[test]
fn test_visit_bool_false() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = IgnoredAny;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Test visitor expecting")
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_bool::<()>(false);
    assert_eq!(result, Ok(IgnoredAny));
}

