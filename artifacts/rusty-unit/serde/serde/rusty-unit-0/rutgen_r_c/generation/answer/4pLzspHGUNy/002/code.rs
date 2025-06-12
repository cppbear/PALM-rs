// Answer 0

#[test]
fn test_deserialize_any_ok() {
    struct TestVisitor {
        value: Option<u32>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = u32;

        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>
        {
            Ok(self.value.unwrap())
        }
    }

    struct DummyDeserializer {
        count: usize,
        seq_done: bool,
    }

    impl<'de> de::Deserializer<'de> for DummyDeserializer {
        type Error = ();
        
        fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            let v = visitor.visit_seq(&mut self)?;
            self.end()?;
            Ok(v)
        }

        fn end(self) -> Result<(), Self::Error> {
            if !self.seq_done {
                Err(())
            } else {
                Ok(())
            }
        }
    }
    
    let deserializer = DummyDeserializer {
        count: 1,
        seq_done: true,
    };
    let visitor = TestVisitor { value: Some(42) };
    let result: Result<u32, ()> = deserializer.deserialize_any(visitor);
    assert_eq!(result, Ok(42));
}

#[test]
fn test_deserialize_any_err() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>
        {
            Ok(())
        }
    }

    struct DummyDeserializer {
        count: usize,
        seq_done: bool,
    }

    impl<'de> de::Deserializer<'de> for DummyDeserializer {
        type Error = ();
        
        fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            let v = visitor.visit_seq(&mut self)?;
            self.end()?;
            Ok(v)
        }

        fn end(self) -> Result<(), Self::Error> {
            if !self.seq_done {
                Err(())
            } else {
                Ok(())
            }
        }
    }
    
    let deserializer = DummyDeserializer {
        count: 1,
        seq_done: false,
    };
    let visitor = TestVisitor;
    let result: Result<(), ()> = deserializer.deserialize_any(visitor);
    assert_eq!(result, Err(()));
}

