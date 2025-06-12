// Answer 0

#[test]
fn test_visit_i128() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = IgnoredAny;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "any value")
        }
    }

    let visitor = TestVisitor;
    let result: Result<IgnoredAny, _> = visitor.visit_i128(42i128);
    assert_eq!(result, Ok(IgnoredAny));
}

#[test]
fn test_visit_i128_negative() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = IgnoredAny;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "any value")
        }
    }

    let visitor = TestVisitor;
    let result: Result<IgnoredAny, _> = visitor.visit_i128(-42i128);
    assert_eq!(result, Ok(IgnoredAny));
}

