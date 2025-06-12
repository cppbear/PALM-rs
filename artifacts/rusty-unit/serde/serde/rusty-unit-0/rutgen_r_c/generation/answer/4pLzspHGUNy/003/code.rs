// Answer 0

#[test]
fn test_deserialize_any_success() {
    use std::marker::PhantomData;
    use std::iter;

    struct MockVisitor {
        expected_len: usize,
        called: bool,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = usize;

        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            self.called = true;
            Ok(self.expected_len)
        }
    }

    struct MockIterator {
        count: usize,
    }

    impl Iterator for MockIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(1) // Dummy value
            } else {
                None
            }
        }
    }

    impl<'de> de::Deserializer<'de> for SeqDeserializer<MockIterator, ()> {
        type Error = ();
        fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            let v = visitor.visit_seq(&mut self)?;
            self.end()?;
            Ok(v)
        }
    }

    let iter = MockIterator { count: 0 }; // No remaining elements
    let deserializer = SeqDeserializer {
        iter: iter.fuse(),
        count: 0,
        marker: PhantomData,
    };
    let visitor = MockVisitor {
        expected_len: 0,
        called: false,
    };

    let result: Result<usize, ()> = deserializer.deserialize_any(visitor);
    assert!(result.is_ok());
    assert!(visitor.called);
}

#[test]
#[should_panic]
fn test_deserialize_any_end_failure() {
    use std::marker::PhantomData;
    use std::iter;

    struct MockVisitor {
        called: bool,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = usize;

        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            self.called = true;
            Ok(1) // Dummy value
        }
    }

    struct MockIterator {
        count: usize,
    }

    impl Iterator for MockIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(1) // Dummy value
            } else {
                None
            }
        }
    }

    impl<'de> de::Deserializer<'de> for SeqDeserializer<MockIterator, ()> {
        type Error = ();
        fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            let v = visitor.visit_seq(&mut self)?;
            self.end()?;
            Ok(v)
        }
    }

    let iter = MockIterator { count: 1 }; // Remains one element
    let deserializer = SeqDeserializer {
        iter: iter.fuse(),
        count: 0,
        marker: PhantomData,
    };
    let visitor = MockVisitor { called: false };

    // This will panic because the end will try to validate remaining items
    deserializer.deserialize_any(visitor);
}

