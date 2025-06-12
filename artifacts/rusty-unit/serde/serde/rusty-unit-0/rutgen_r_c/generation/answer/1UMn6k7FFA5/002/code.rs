// Answer 0

#[test]
fn test_newtype_variant_seed_none() {
    struct MockError;
    impl de::Error for MockError {
        fn custom<T>(_msg: T) -> Self {
            MockError
        }
    }

    struct MockSeed;
    impl<'de> de::DeserializeSeed<'de> for MockSeed {
        type Value = ();
        fn deserialize<V>(self, _: V) -> Result<Self::Value, MockError>
        where
            V: de::Deserializer<'de>,
        {
            unimplemented!()
        }
    }

    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a newtype variant")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(())
        }
    }

    struct VariantAccessMock {
        value: Option<&'static Content<'static>>,
    }

    impl<'de> de::VariantAccess<'de> for VariantAccessMock {
        type Error = MockError;

        fn unit_variant(self) -> Result<(), Self::Error> {
            Ok(())
        }

        fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
        where
            T: de::DeserializeSeed<'de>,
        {
            match self.value {
                Some(value) => seed.deserialize(ContentRefDeserializer::new(value)),
                None => Err(de::Error::invalid_type(
                    de::Unexpected::UnitVariant,
                    &"newtype variant",
                )),
            }
        }

        fn tuple_variant<V>(self, _: usize, _: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            unimplemented!()
        }

        fn struct_variant<V>(self, _: &'static [&'static str], _: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            unimplemented!()
        }
    }

    let deserializer = VariantAccessMock { value: None };

    let result = deserializer.newtype_variant_seed(MockSeed);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), MockError);
}

