// Answer 0

#[test]
fn test_visit_bool_true() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Content<'de>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a boolean value")
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_bool(true);
    assert_eq!(result, Ok(Content::Bool(true)));
}

#[test]
fn test_visit_bool_false() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Content<'de>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a boolean value")
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_bool(false);
    assert_eq!(result, Ok(Content::Bool(false)));
}

