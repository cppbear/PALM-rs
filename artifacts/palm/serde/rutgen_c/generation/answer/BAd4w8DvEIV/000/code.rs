// Answer 0

#[test]
fn test_struct_variant_with_map() {
    use crate::de::{self, Visitor};

    struct TestVisitor {
        value: Option<u32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<u32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map with a u32 value")
        }

        fn visit_map<M>(self, mut visitor: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            let mut value = None;

            while let Some((key, val)) = visitor.next_entry::<Content<'de>, Content<'de>>()? {
                if let Content::Str(ref k) = key {
                    if k == "value" {
                        if let Content::U32(v) = *val {
                            value = Some(v);
                        }
                    }
                }
            }
            Ok(value)
        }
    }

    let content = Some(Content::Map(vec![
        (Content::Str("value".to_string()), Content::U32(42)),
    ]));

    let deserializer = VariantRefDeserializer {
        value: content,
        err: PhantomData,
    };

    let result: Option<u32> = deserializer.struct_variant(&["value"], TestVisitor { value: None }).unwrap();
    assert_eq!(result, Some(42));
}

#[test]
fn test_struct_variant_with_seq() {
    use crate::de::{self, Visitor};

    struct TestVisitor {
        value: Vec<u32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of u32 values")
        }

        fn visit_seq<S>(self, mut visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(value) = visitor.next_element::<Content<'de>>()? {
                if let Content::U32(v) = *value {
                    values.push(v);
                }
            }
            Ok(values)
        }
    }

    let content = Some(Content::Seq(vec![
        Content::U32(1),
        Content::U32(2),
        Content::U32(3),
    ]));

    let deserializer = VariantRefDeserializer {
        value: content,
        err: PhantomData,
    };

    let result: Vec<u32> = deserializer.struct_variant(&["value"], TestVisitor { value: Vec::new() }).unwrap();
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_struct_variant_with_unexpected_type() {
    use crate::de;

    let content = Some(Content::Str("unexpected".to_string()));

    let deserializer = VariantRefDeserializer {
        value: content,
        err: PhantomData,
    };

    let result: Result<(), de::Error> = deserializer.struct_variant(&["value"], TestVisitor { value: Vec::new() });
    assert!(result.is_err());
}

#[test]
fn test_struct_variant_with_none() {
    use crate::de;

    let content: Option<Content> = None;

    let deserializer = VariantRefDeserializer {
        value: content,
        err: PhantomData,
    };

    let result: Result<(), de::Error> = deserializer.struct_variant(&["value"], TestVisitor { value: Vec::new() });
    assert!(result.is_err());
}

