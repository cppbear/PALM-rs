// Answer 0

#[test]
fn test_deserialize_struct_with_seq() {
    struct TestVisitor {
        value: Vec<u8>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_seq<V>(self, visitor: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(value) = visitor.next_element::<u8>()? {
                values.push(value);
            }
            Ok(values)
        }

        // Implement other Visitor methods as needed, only visit_seq is shown here to keep it brief.
    }

    let content = Content::Seq(vec![Content::U8(1), Content::U8(2), Content::U8(3)]);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = TestVisitor { value: Vec::new() };

    let result: Result<Vec<u8>, _> = deserializer.deserialize_struct("TestStruct", &[], visitor);
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_deserialize_struct_with_map() {
    struct TestVisitor {
        key_value_pairs: Vec<(u8, String)>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<(u8, String)>;

        fn visit_map<V>(self, visitor: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'de>,
        {
            let mut pairs = Vec::new();
            while let Some((key, value)) = visitor.next_entry::<u8, String>()? {
                pairs.push((key, value));
            }
            Ok(pairs)
        }

        // Implement other Visitor methods as needed, only visit_map is shown here to keep it brief.
    }

    let content = Content::Map(vec![
        (Content::U8(1), Content::String("value1".to_string())),
        (Content::U8(2), Content::String("value2".to_string())),
    ]);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = TestVisitor { key_value_pairs: Vec::new() };

    let result: Result<Vec<(u8, String)>, _> = deserializer.deserialize_struct("TestStruct", &[], visitor);
    assert_eq!(result.unwrap(), vec![(1, "value1".to_string()), (2, "value2".to_string())]);
}

#[test]
#[should_panic]
fn test_deserialize_struct_with_invalid_content() {
    struct TestVisitor {
        value: Vec<u8>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            panic!("This should not be called");
        }

        fn visit_map<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'de>,
        {
            panic!("This should not be called");
        }
    }

    let content = Content::U8(42);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = TestVisitor { value: Vec::new() };

    let result: Result<Vec<u8>, _> = deserializer.deserialize_struct("TestStruct", &[], visitor);
    result.unwrap(); // Should panic before this line is reached
}

