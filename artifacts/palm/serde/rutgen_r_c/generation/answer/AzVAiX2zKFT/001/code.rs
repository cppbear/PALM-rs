// Answer 0

#[test]
fn test_visit_u128() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = IgnoredAny;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "TestVisitor expecting")
        }

        fn visit_u128<E>(self, x: u128) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let _ = x;
            Ok(IgnoredAny)
        }
    }

    let visitor = TestVisitor;

    let result = visitor.visit_u128(0u128);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), IgnoredAny);

    let result = visitor.visit_u128(1u128);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), IgnoredAny);

    let result = visitor.visit_u128(u128::MAX);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), IgnoredAny);
}

