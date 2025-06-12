// Answer 0

#[test]
fn test_visit_i64() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = IgnoredAny;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a test visitor")
        }
    }

    let visitor = TestVisitor;
    let result: Result<IgnoredAny, ()> = visitor.visit_i64(10);
    assert_eq!(result, Ok(IgnoredAny));
}

#[test]
fn test_visit_i64_negative() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = IgnoredAny;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a test visitor")
        }
    }

    let visitor = TestVisitor;
    let result: Result<IgnoredAny, ()> = visitor.visit_i64(-10);
    assert_eq!(result, Ok(IgnoredAny));
}

#[test]
fn test_visit_i64_boundary() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = IgnoredAny;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a test visitor")
        }
    }

    let visitor = TestVisitor;
    let result: Result<IgnoredAny, ()> = visitor.visit_i64(i64::MIN);
    assert_eq!(result, Ok(IgnoredAny));

    let result: Result<IgnoredAny, ()> = visitor.visit_i64(i64::MAX);
    assert_eq!(result, Ok(IgnoredAny));
}

