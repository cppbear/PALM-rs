// Answer 0

#[test]
fn test_visit_unit_returns_ignored_any() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = IgnoredAny;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "test visitor expecting")
        }
    }

    let visitor = TestVisitor;
    let result: Result<IgnoredAny, ()> = visitor.visit_unit();
    assert_eq!(result, Ok(IgnoredAny));
}

