// Answer 0

#[test]
fn test_visit_unit() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<()>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an option")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }
    }

    let visitor = TestVisitor;

    assert_eq!(visitor.visit_unit::<std::convert::Infallible>().ok(), Some(None));
}

#[test]
fn test_visit_none() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<()>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an option")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }
    }

    let visitor = TestVisitor;

    assert_eq!(visitor.visit_none::<std::convert::Infallible>().ok(), Some(None));
}

#[test]
#[should_panic]
fn test_visit_some_panics() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<()>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an option")
        }

        fn visit_some<D>(self, _: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            panic!("this should not be called")
        }
    }

    let visitor = TestVisitor;
    let some_result = visitor.visit_some(());
    assert!(some_result.is_err());
}

