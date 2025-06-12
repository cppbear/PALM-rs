// Answer 0

#[test]
fn test_deserialize_any_with_empty_map() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_map<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    struct TestIterator {
        count: usize,
    }

    impl Iterator for TestIterator {
        type Item = (&'static str, &'static str);

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let iter = TestIterator { count: 0 };
    let map_deserializer = MapDeserializer {
        iter: iter.fuse(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let _: Result<_, ()> = map_deserializer.deserialize_any(TestVisitor);
}

#[test]
fn test_deserialize_any_with_single_entry() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_map<V>(self, _: &mut V) -> Result<Self::Value, V::Error> {
            Ok(())
        }
    }

    struct TestIterator {
        count: usize,
    }

    impl Iterator for TestIterator {
        type Item = (&'static str, &'static str);

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 1 {
                self.count += 1;
                Some(("key", "value"))
            } else {
                None
            }
        }
    }

    let iter = TestIterator { count: 0 };
    let map_deserializer = MapDeserializer {
        iter: iter.fuse(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let _: Result<_, ()> = map_deserializer.deserialize_any(TestVisitor);
}

#[test]
fn test_deserialize_any_with_multiple_entries() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_map<V>(self, _: &mut V) -> Result<Self::Value, V::Error> {
            Ok(())
        }
    }

    struct TestIterator {
        count: usize,
        limit: usize,
    }

    impl Iterator for TestIterator {
        type Item = (&'static str, &'static str);

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.limit {
                self.count += 1;
                Some(("key", "value"))
            } else {
                None
            }
        }
    }

    let iter = TestIterator { count: 0, limit: 2 };
    let map_deserializer = MapDeserializer {
        iter: iter.fuse(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let _: Result<_, ()> = map_deserializer.deserialize_any(TestVisitor);
}

