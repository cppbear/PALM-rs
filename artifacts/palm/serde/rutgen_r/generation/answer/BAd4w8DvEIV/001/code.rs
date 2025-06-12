// Answer 0

#[test]
fn test_struct_variant_invalid_type_some_other() {
    struct TestVisitor;
    
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct variant")
        }
        
        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
            Err(de::Error::custom("unexpected bool"))
        }
    }

    struct TestStruct {
        value: Option<Content>,
    }

    impl TestStruct {
        fn struct_variant<V>(
            self,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, de::Error>
        where
            V: de::Visitor<'de>,
        {
            match self.value {
                Some(Content::Map(v)) => visit_content_map_ref(v, visitor),
                Some(Content::Seq(v)) => visit_content_seq_ref(v, visitor),
                Some(other) => Err(de::Error::invalid_type(
                    other.unexpected(),
                    &"struct variant",
                )),
                None => Err(de::Error::invalid_type(
                    de::Unexpected::UnitVariant,
                    &"struct variant",
                )),
            }
        }
    }

    let test_case = TestStruct {
        value: Some(Content::Bool(true)), // Some(other)
    };

    let result: Result<(), de::Error> = test_case.struct_variant(&[], TestVisitor);
    assert!(result.is_err());
}

#[test]
fn test_struct_variant_invalid_type_none() {
    struct TestVisitor;
    
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct variant")
        }
        
        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
            Err(de::Error::custom("unexpected bool"))
        }
    }

    struct TestStruct {
        value: Option<Content>,
    }

    impl TestStruct {
        fn struct_variant<V>(
            self,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, de::Error>
        where
            V: de::Visitor<'de>,
        {
            match self.value {
                Some(Content::Map(v)) => visit_content_map_ref(v, visitor),
                Some(Content::Seq(v)) => visit_content_seq_ref(v, visitor),
                Some(other) => Err(de::Error::invalid_type(
                    other.unexpected(),
                    &"struct variant",
                )),
                None => Err(de::Error::invalid_type(
                    de::Unexpected::UnitVariant,
                    &"struct variant",
                )),
            }
        }
    }

    let test_case = TestStruct {
        value: None, // None
    };

    let result: Result<(), de::Error> = test_case.struct_variant(&[], TestVisitor);
    assert!(result.is_err());
}

