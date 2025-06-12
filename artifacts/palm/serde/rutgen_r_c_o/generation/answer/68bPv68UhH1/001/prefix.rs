// Answer 0

#[test]
fn test_deserialize_ignored_any_valid() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(())
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("unit")
        }
    }

    let mut data: Vec<Option<(Content, Content)>> = Vec::new();
    let deserializer = FlatMapDeserializer(&mut data, PhantomData::<()>);
    let visitor = TestVisitor;

    let _ = deserializer.deserialize_ignored_any(visitor);
}

#[should_panic]
fn test_deserialize_ignored_any_invalid_visitor() {
    struct InvalidVisitor;

    impl<'de> Visitor<'de> for InvalidVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            panic!("Visitor should not be called");
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("unit")
        }
    }

    let mut data: Vec<Option<(Content, Content)>> = Vec::new();
    let deserializer = FlatMapDeserializer(&mut data, PhantomData::<()>);
    let visitor = InvalidVisitor;

    let _ = deserializer.deserialize_ignored_any(visitor);
}

