// Answer 0

#[test]
fn test_struct_variant_with_map() {
    struct VisitorImpl;
    
    impl<'de> de::Visitor<'de> for VisitorImpl {
        type Value = ();
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }
        
        fn visit_map<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    let value = Some(Content::Map(vec![
        (Content::String("key1".to_string()), Content::U32(10)),
        (Content::String("key2".to_string()), Content::U32(20)),
    ]));

    let deserializer = VariantRefDeserializer {
        value,
        err: PhantomData::<Box<dyn de::Error>>,
    };

    deserializer.struct_variant(&["key1", "key2"], VisitorImpl);
}

#[test]
fn test_struct_variant_with_seq() {
    struct VisitorImpl;
    
    impl<'de> de::Visitor<'de> for VisitorImpl {
        type Value = ();
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }
        
        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let value = Some(Content::Seq(vec![
        Content::U32(1),
        Content::U32(2),
        Content::U32(3),
    ]));

    let deserializer = VariantRefDeserializer {
        value,
        err: PhantomData::<Box<dyn de::Error>>,
    };

    deserializer.struct_variant(&[], VisitorImpl);
}

#[test]
#[should_panic]
fn test_struct_variant_with_other_content() {
    struct VisitorImpl;

    impl<'de> de::Visitor<'de> for VisitorImpl {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
            Ok(())
        }
    }

    let value = Some(Content::Bool(true));

    let deserializer = VariantRefDeserializer {
        value,
        err: PhantomData::<Box<dyn de::Error>>,
    };

    deserializer.struct_variant(&[], VisitorImpl);
}

#[test]
#[should_panic]
fn test_struct_variant_with_none() {
    struct VisitorImpl;

    impl<'de> de::Visitor<'de> for VisitorImpl {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(())
        }
    }

    let value: Option<Content> = None;

    let deserializer = VariantRefDeserializer {
        value,
        err: PhantomData::<Box<dyn de::Error>>,
    };

    deserializer.struct_variant(&[], VisitorImpl);
}

