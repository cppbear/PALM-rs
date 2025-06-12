// Answer 0

#[test]
fn test_unit_variant_none() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(_: T) -> Self {
            TestError
        }
    }

    struct VariantRefDeserializer<'a, 'de: 'a> {
        value: Option<&'a Content<'de>>,
    }

    impl<'de, 'a> de::VariantAccess<'de> for VariantRefDeserializer<'a, 'de> {
        type Error = TestError;

        fn unit_variant(self) -> Result<(), Self::Error> {
            match self.value {
                Some(value) => de::Deserialize::deserialize(ContentRefDeserializer::new(value)).map_err(|_| TestError),
                None => Ok(()),
            }
        }
        
        fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error> 
        where T: de::DeserializeSeed<'de> {
            unimplemented!()
        }
        
        fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error> 
        where V: de::Visitor<'de> {
            unimplemented!()
        }
        
        fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> 
        where V: de::Visitor<'de> {
            unimplemented!()
        }
    }

    let deserializer = VariantRefDeserializer { value: None };

    let result: Result<(), TestError> = deserializer.unit_variant();
    assert!(result.is_ok());
}

