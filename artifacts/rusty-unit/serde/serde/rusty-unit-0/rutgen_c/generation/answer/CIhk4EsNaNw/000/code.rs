// Answer 0

#[test]
fn test_visit_enum() {
    struct MockEnumAccess<'de> {
        data: &'de str,
    }

    impl<'de> EnumAccess<'de> for MockEnumAccess<'de> {
        type Error = &'static str;

        fn variant_seed(self) -> Result<(std::string::String, Self::Variant), Self::Error> {
            Ok((self.data.to_string(), MockVariant))
        }
    }

    struct MockVariant;

    impl<'de> de::VariantAccess<'de> for MockVariant {
        type Error = &'static str;

        fn unit_variant(self) -> Result<(), Self::Error> {
            Ok(())
        }

        fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            Err("Encountered a newtype variant")
        }

        fn tuple_variant<V>(
            self,
            _len: usize,
            _visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: SeqAccess<'de>,
        {
            Err("Encountered a tuple variant")
        }

        fn struct_variant<V>(
            self,
            _fields: &'static [&'static str],
            _visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: MapAccess<'de>,
        {
            Err("Encountered a struct variant")
        }
    }

    let visitor = MockEnumAccess { data: "Test" };
    let result = TagOrContentVisitor {
        name: "Test",
        value: PhantomData,
    }
    .visit_enum(visitor);

    assert!(result.is_ok());
}

