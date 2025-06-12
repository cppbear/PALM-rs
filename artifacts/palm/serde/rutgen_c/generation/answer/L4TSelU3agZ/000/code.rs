// Answer 0

#[test]
fn test_visit_unit() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = PhantomData<()>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("unit value")
        }

        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(PhantomData)
        }
    }

    let visitor = TestVisitor;
    let result: Result<PhantomData<()>, ()> = visitor.visit_unit();
    assert!(result.is_ok());
}

#[test]
fn test_visit_unit_error() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = PhantomData<()>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("unit value")
        }

        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(())
        }
    }

    let visitor = TestVisitor;
    let result: Result<PhantomData<()>, ()> = visitor.visit_unit();
    assert!(result.is_err());
}

