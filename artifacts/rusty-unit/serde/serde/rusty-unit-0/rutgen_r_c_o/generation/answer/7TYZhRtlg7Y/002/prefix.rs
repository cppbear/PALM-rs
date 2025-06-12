// Answer 0

#[test]
fn test_deserialize_struct_with_map() {
    let content = Content::Map(vec![
        (Content::String("key1".to_string()), Content::U32(1)),
        (Content::String("key2".to_string()), Content::U32(2)),
    ]);
    
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    
    deserializer.deserialize_struct("TestStruct", &["key1", "key2"], DummyVisitor);
}

#[test]
fn test_deserialize_struct_with_nested_map() {
    let content = Content::Map(vec![
        (
            Content::String("outer_key".to_string()),
            Content::Map(vec![
                (Content::String("inner_key1".to_string()), Content::U32(10)),
                (Content::String("inner_key2".to_string()), Content::U32(20)),
            ]),
        ),
    ]);
    
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    
    deserializer.deserialize_struct("OuterStruct", &["outer_key"], DummyVisitor);
}

#[test]
fn test_deserialize_struct_with_empty_map() {
    let content = Content::Map(vec![]);
    
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    
    deserializer.deserialize_struct("EmptyStruct", &[], DummyVisitor);
}

#[test]
fn test_deserialize_struct_with_multiple_key_types() {
    let content = Content::Map(vec![
        (Content::String("mixed_key".to_string()), Content::Seq(vec![
            Content::U32(1),
            Content::U32(2),
        ])),
    ]);
    
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    
    deserializer.deserialize_struct("MixedKeyStruct", &["mixed_key"], DummyVisitor);
}

#[test]
fn test_deserialize_struct_with_non_string_key() {
    let content = Content::Map(vec![
        (Content::U32(42), Content::U32(100)),
    ]);
    
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    
    let result = deserializer.deserialize_struct("InvalidKeyStruct", &[], DummyVisitor);
    assert!(result.is_err());
}

struct DummyVisitor;

impl<'de> Visitor<'de> for DummyVisitor {
    type Value = ();
    
    fn visit_map<V>(self, _: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        Ok(())
    }
    
    fn visit_seq<V>(self, _: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        Ok(())
    }
    
    fn visit_unit(self) -> Result<Self::Value, Self::Error> {
        Ok(())
    }
}

