// Answer 0

#[test]
fn test_deserialize_seq_valid() {
    struct TestVisitor {
        count: usize,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = usize;

        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            Ok(self.count)
        }
    }

    let items = vec![1, 2, 3];
    let deserializer = SeqDeserializer {
        iter: items.into_iter().fuse(),
        count: 3,
        marker: PhantomData,
    };

    let visitor = TestVisitor { count: 3 };
    let _ = deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_seq_empty() {
    struct TestVisitor {
        count: usize,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = usize;

        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            Ok(self.count)
        }
    }

    let items: Vec<i32> = Vec::new();
    let deserializer = SeqDeserializer {
        iter: items.into_iter().fuse(),
        count: 0,
        marker: PhantomData,
    };

    let visitor = TestVisitor { count: 0 };
    let _ = deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_seq_length_mismatch() {
    struct TestVisitor {
        count: usize,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = usize;

        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            Err(de::Error::custom("length mismatch"))
        }
    }

    let items = vec![1, 2, 3, 4];
    let deserializer = SeqDeserializer {
        iter: items.into_iter().fuse(),
        count: 2,
        marker: PhantomData,
    };

    let visitor = TestVisitor { count: 4 };
    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_seq_non_empty() {
    struct TestVisitor {
        count: usize,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = usize;

        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            Ok(self.count)
        }
    }

    let items = vec![10, 20, 30, 40];
    let deserializer = SeqDeserializer {
        iter: items.into_iter().fuse(),
        count: 4,
        marker: PhantomData,
    };

    let visitor = TestVisitor { count: 4 };
    let _ = deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_seq_with_size_hint() {
    struct TestVisitor {
        count: usize,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = usize;

        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            Ok(self.count)
        }
    }

    let items = vec![5, 15, 25];
    let deserializer = SeqDeserializer {
        iter: items.into_iter().fuse(),
        count: 3,
        marker: PhantomData,
    };

    let visitor = TestVisitor { count: 3 };
    let _ = deserializer.deserialize_any(visitor);
}

