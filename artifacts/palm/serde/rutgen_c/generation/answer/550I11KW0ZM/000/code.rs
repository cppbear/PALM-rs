// Answer 0

#[test]
fn test_deserialize_struct_with_seq() {
    struct DummyVisitor {
        value: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = Vec<u8>;

        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, ()>
        where
            V: SeqAccess<'de>,
        {
            Ok(self.value.unwrap_or_default())
        }

        // Other required methods can be left unimplemented for a minimal test.
        fn visit_map<V>(self, _visitor: V) -> Result<Self::Value, ()>
        where
            V: MapAccess<'de>,
        {
            Err(())
        }

        fn visit_unit(self) -> Result<Self::Value, ()> {
            Err(())
        }
    }

    let content = Content::Seq(vec![Content::U8(1), Content::U8(2)]);
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let visitor = DummyVisitor {
        value: Some(vec![1, 2]),
    };

    let result: Result<Vec<u8>, _> = deserializer.deserialize_struct("dummy", &[], visitor);
    assert_eq!(result.unwrap(), vec![1, 2]);
}

#[test]
fn test_deserialize_struct_with_map() {
    struct DummyVisitor {
        value: Option<u32>,
    }

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = u32;

        fn visit_map<V>(self, _visitor: V) -> Result<Self::Value, ()>
        where
            V: MapAccess<'de>,
        {
            Ok(self.value.unwrap_or_default())
        }

        // Other required methods can be left unimplemented for a minimal test.
        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, ()>
        where
            V: SeqAccess<'de>,
        {
            Err(())
        }

        fn visit_unit(self) -> Result<Self::Value, ()> {
            Err(())
        }
    }

    let content = Content::Map(vec![(Content::String("key".into()), Content::U32(42))]);
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let visitor = DummyVisitor {
        value: Some(42),
    };

    let result: Result<u32, _> = deserializer.deserialize_struct("dummy", &[], visitor);
    assert_eq!(result.unwrap(), 42);
}

#[test]
#[should_panic]
fn test_deserialize_struct_invalid_type() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = ();

        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, ()> 
        where
            V: SeqAccess<'de> {
                Err(())
        }

        fn visit_map<V>(self, _visitor: V) -> Result<Self::Value, ()>
        where
            V: MapAccess<'de> {
                Err(())
        }
    }

    let content = Content::Bool(true);
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let visitor = DummyVisitor;

    let _: Result<(), _> = deserializer.deserialize_struct("dummy", &[], visitor);
}

