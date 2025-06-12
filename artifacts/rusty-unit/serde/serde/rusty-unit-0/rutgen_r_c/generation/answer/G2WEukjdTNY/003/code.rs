// Answer 0

#[test]
fn test_tuple_variant_none() {
    struct TestVisitor;
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a tuple variant")
        }
        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            unimplemented!()
        }
    }

    struct TestVariantAccess<'de> {
        value: Option<Content<'de>>,
    }

    impl<'de> VariantAccess<'de> for TestVariantAccess<'de> {
        type Error = std::convert::Infallible;
        fn unit_variant(self) -> Result<(), Self::Error> {
            unimplemented!()
        }
        fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            unimplemented!()
        }
        fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.value {
                Some(Content::Seq(v)) => visit_content_seq_ref(v, visitor),
                Some(other) => {
                    Err(de::Error::invalid_type(other.unexpected(), &"tuple variant"))
                }
                None => {
                    Err(de::Error::invalid_type(
                        de::Unexpected::UnitVariant,
                        &"tuple variant",
                    ))
                }
            }
        }
        fn struct_variant<V>(
            self,
            _fields: &'static [&'static str],
            _visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }
    }

    let test_access = TestVariantAccess { value: None };
    let result: Result<(), _> = test_access.tuple_variant(0, TestVisitor);
    assert!(result.is_err());
}

#[test]
fn test_tuple_variant_wrong_type() {
    struct TestVisitor;
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a tuple variant")
        }
        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            unimplemented!()
        }
    }

    let test_access = TestVariantAccess {
        value: Some(Content::String("test".into())),
    };
    let result: Result<(), _> = test_access.tuple_variant(0, TestVisitor);
    assert!(result.is_err());
}

