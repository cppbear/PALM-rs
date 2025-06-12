// Answer 0

#[test]
fn test_deserialize_any_should_return_err_on_end_with_remaining_items() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();
        
        fn visit_map<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    struct MockIterator;

    impl Iterator for MockIterator {
        type Item = (i32, i32);

        fn next(&mut self) -> Option<Self::Item> {
            None // No items left to iterate
        }

        fn count(self) -> usize {
            1 // Simulate that there is one remaining item
        }
    }

    struct MockError;

    impl de::Error for MockError {}

    let deserializer = MapDeserializer {
        iter: MockIterator.fuse(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let result: Result<(), _> = deserializer.deserialize_any(MockVisitor);
}

#[test]
fn test_deserialize_any_should_return_err_on_end_with_multiple_remaining_items() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();
        
        fn visit_map<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    struct MockIterator;

    impl Iterator for MockIterator {
        type Item = (i32, i32);

        fn next(&mut self) -> Option<Self::Item> {
            None // No items left to iterate
        }

        fn count(self) -> usize {
            5 // Simulate that there are five remaining items
        }
    }

    struct MockError;

    impl de::Error for MockError {}

    let deserializer = MapDeserializer {
        iter: MockIterator.fuse(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let result: Result<(), _> = deserializer.deserialize_any(MockVisitor);
}

