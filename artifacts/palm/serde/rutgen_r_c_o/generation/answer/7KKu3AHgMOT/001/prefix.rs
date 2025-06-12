// Answer 0

#[test]
fn test_deserialize_option_valid() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_none(self) -> Result<Self::Value, Box<str>> {
            Ok(())
        }

        fn visit_some<D: de::Deserializer<'de>>(self, _: D) -> Result<Self::Value, Box<str>> {
            Err(Box::from("Should not visit some"))
        }
    }

    let deserializer = UnitDeserializer::<Box<str>> { marker: PhantomData };
    let visitor = TestVisitor;
    let _ = deserializer.deserialize_option(visitor);
}

#[test]
#[should_panic]
fn test_deserialize_option_invalid_visit_some() {
    struct InvalidVisitor;

    impl<'de> de::Visitor<'de> for InvalidVisitor {
        type Value = ();

        fn visit_none(self) -> Result<Self::Value, Box<str>> {
            Err(Box::from("Should not visit none"))
        }

        fn visit_some<D: de::Deserializer<'de>>(self, _: D) -> Result<Self::Value, Box<str>> {
            Ok(())
        }
    }

    let deserializer = UnitDeserializer::<Box<str>> { marker: PhantomData };
    let visitor = InvalidVisitor;
    let _ = deserializer.deserialize_option(visitor);
}

