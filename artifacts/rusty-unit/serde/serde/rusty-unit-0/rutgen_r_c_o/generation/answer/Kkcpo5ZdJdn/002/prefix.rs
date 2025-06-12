// Answer 0

#[test]
fn test_unit_variant_none() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(_: T) -> Self { TestError }
    }

    struct TestVariantAccess<'de> {
        value: Option<&'de Content<'de>>,
    }

    impl<'de> de::VariantAccess<'de> for TestVariantAccess<'de> {
        type Error = TestError;
        
        fn unit_variant(self) -> Result<(), Self::Error> {
            match self.value {
                Some(value) => de::Deserialize::deserialize(ContentRefDeserializer::new(value)),
                None => Ok(()),
            }
        }

        fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error> {
            Ok(Default::default())
        }

        fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>
        {
            Err(TestError)
        }

        fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>
        {
            Err(TestError)
        }
    }

    let variant_access = TestVariantAccess { value: None };
    variant_access.unit_variant().unwrap();
}

