// Answer 0

#[test]
fn test_struct_variant_with_map() {
    use crate::de::{Visitor, Error};

    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'de>,
        {
            let mut result = String::new();
            while let Some((key, value)) = map.next_entry()? {
                result.push_str(&format!("{}: {:?}; ", key.as_str().unwrap(), value));
            }
            Ok(result)
        }
    }

    let content = Content::Map(vec![
        (Content::String("key1".into()), Content::U32(10)),
        (Content::String("key2".into()), Content::U32(20)),
    ]);
    let deserializer = VariantRefDeserializer {
        value: Some(&content),
        err: PhantomData,
    };

    let result = deserializer.struct_variant(&["key1", "key2"], TestVisitor { value: None });
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "key1: U32(10); key2: U32(20); ");
}

#[test]
fn test_struct_variant_with_sequence() {
    use crate::de::{Visitor, Error};

    struct TestVisitor {
        value: Option<Vec<u32>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut result = Vec::new();
            while let Some(value) = seq.next_element()? {
                result.push(value);
            }
            Ok(result)
        }
    }

    let content = Content::Seq(vec![Content::U32(10), Content::U32(20)]);
    let deserializer = VariantRefDeserializer {
        value: Some(&content),
        err: PhantomData,
    };

    let result = deserializer.struct_variant(&[], TestVisitor { value: None });
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![10, 20]);
}

#[test]
fn test_struct_variant_with_invalid_type() {
    use crate::de::{Visitor, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            Err(de::Error::custom("not a valid type"))
        }

        fn visit_map<V>(self, _: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'de>,
        {
            Err(de::Error::custom("not a valid type"))
        }
    }

    let content = Content::I32(42); // Invalid type for a struct variant
    let deserializer = VariantRefDeserializer {
        value: Some(&content),
        err: PhantomData,
    };

    let result = deserializer.struct_variant(&["field1"], TestVisitor);
    assert!(result.is_err());
}

