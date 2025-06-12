// Answer 0

#[test]
fn test_deserialize_any_with_seq() {
    struct TestVisitor {
        value: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_seq<V>(self, seq: &mut V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(item) = seq.next_element::<u8>()? {
                values.push(item);
            }
            Ok(values)
        }

        fn visit_bytes(self, _value: &'de [u8]) -> Result<Self::Value, Self::Error> {
            self.value = Some(_value.to_vec());
            Ok(self.value.unwrap())
        }

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(vec![])
        }
        
        // other visitor methods can be implemented with minimal no-op functionality.
        fn visit_bool(self, _value: bool) -> Result<Self::Value, Self::Error> {
            Ok(vec![])
        }
        
        fn visit_u8(self, _value: u8) -> Result<Self::Value, Self::Error> {
            Ok(vec![_value])
        }

        fn visit_none(self) -> Result<Self::Value, Self::Error> {
            Ok(vec![])
        }

        fn visit_some<V>(self, _value: V) -> Result<Self::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            Ok(vec![])
        }
    }

    let content = Content::Seq(vec![Content::U8(1), Content::U8(2), Content::U8(3)]);
    let deserializer = ContentDeserializer::new(content);
    let visitor = TestVisitor { value: None };

    let result = deserializer.deserialize_any(visitor);
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_deserialize_any_with_empty_seq() {
    struct EmptyVisitor {}

    impl<'de> Visitor<'de> for EmptyVisitor {
        type Value = Vec<u8>;
        
        fn visit_seq<V>(self, seq: &mut V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(_item) = seq.next_element::<u8>()? {}
            Ok(values)
        }

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(vec![])
        }

        fn visit_none(self) -> Result<Self::Value, Self::Error> {
            Ok(vec![])
        }

        // other visitor methods can be no-ops.
        fn visit_bool(self, _value: bool) -> Result<Self::Value, Self::Error> {
            Ok(vec![])
        }
    }

    let content = Content::Seq(vec![]);
    let deserializer = ContentDeserializer::new(content);
    let visitor = EmptyVisitor {};

    let result = deserializer.deserialize_any(visitor);
    assert_eq!(result.unwrap(), vec![]);
}

