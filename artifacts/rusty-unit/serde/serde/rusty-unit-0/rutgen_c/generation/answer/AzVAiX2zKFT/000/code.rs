// Answer 0

#[test]
fn test_visit_u128() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = IgnoredAny;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "any value")
        }
    }

    let visitor = TestVisitor {};
    let result: Result<IgnoredAny, ()> = visitor.visit_u128(42u128);
    assert_eq!(result, Ok(IgnoredAny));
}

#[test]
fn test_visit_u128_zero() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = IgnoredAny;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "any value")
        }
    }

    let visitor = TestVisitor {};
    let result: Result<IgnoredAny, ()> = visitor.visit_u128(0u128);
    assert_eq!(result, Ok(IgnoredAny));
}

#[test]
fn test_visit_u128_max() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = IgnoredAny;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "any value")
        }
    }

    let visitor = TestVisitor {};
    let result: Result<IgnoredAny, ()> = visitor.visit_u128(u128::MAX);
    assert_eq!(result, Ok(IgnoredAny));
}

