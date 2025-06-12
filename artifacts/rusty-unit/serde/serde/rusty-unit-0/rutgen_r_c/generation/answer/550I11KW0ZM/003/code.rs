// Answer 0

#[test]
fn test_deserialize_struct_with_sequence() {
    use crate::de::Visitor;
    use crate::de::value::ContentDeserializer;

    struct TestVisitor {
        value: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: crate::de::SeqAccess<'de>,
        {
            let mut seq = Vec::new();
            while let Some(value) = visitor.next_element::<u8>()? {
                seq.push(value);
            }
            Ok(seq)
        }

        // Other required visit methods can be implemented as needed
        // but for this test, we focus on visit_seq.
    }

    let content = crate::de::Content::Seq(vec![
        crate::de::Content::U8(10),
        crate::de::Content::U8(20),
        crate::de::Content::U8(30),
    ]);

    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData::<()> {},
    };

    let visitor = TestVisitor { value: None };

    let result = deserializer.deserialize_struct("TestStruct", &[], visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![10, 20, 30]);
}

#[test]
fn test_deserialize_struct_with_empty_sequence() {
    use crate::de::Visitor;
    use crate::de::value::ContentDeserializer;

    struct TestVisitor {
        value: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: crate::de::SeqAccess<'de>,
        {
            let mut seq = Vec::new();
            while let Some(value) = visitor.next_element::<u8>()? {
                seq.push(value);
            }
            Ok(seq)
        }
    }

    let content = crate::de::Content::Seq(vec![]);

    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData::<()> {},
    };

    let visitor = TestVisitor { value: None };

    let result = deserializer.deserialize_struct("TestStruct", &[], visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![]);
}

#[test]
#[should_panic]
fn test_deserialize_struct_with_invalid_content() {
    use crate::de::Visitor;
    use crate::de::value::ContentDeserializer;
    
    struct TestVisitor {
        value: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, V::Error>
        where
            V: crate::de::SeqAccess<'de>
        {
            unreachable!() // Ensure this is never called
        }
    }

    let content = crate::de::Content::Map(vec![]);

    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData::<()> {},
    };

    let visitor = TestVisitor { value: None };
    
    // Expects panic because the content type does not match
    let _ = deserializer.deserialize_struct("TestStruct", &[], visitor);
}

