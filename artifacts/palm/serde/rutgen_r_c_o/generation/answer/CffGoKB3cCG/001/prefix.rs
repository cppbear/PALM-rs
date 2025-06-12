// Answer 0

#[test]
fn test_map_deserializer_into_deserializer() {
    struct TestIterator;

    impl Iterator for TestIterator {
        type Item = (i32, bool);

        fn next(&mut self) -> Option<Self::Item> {
            Some((1, true))
        }
    }

    let deserializer: MapDeserializer<TestIterator, Error> = MapDeserializer {
        iter: TestIterator.fuse(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let _ = deserializer.into_deserializer();
}

#[test]
fn test_map_deserializer_into_deserializer_empty() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = (i32, bool);

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let deserializer: MapDeserializer<EmptyIterator, Error> = MapDeserializer {
        iter: EmptyIterator.fuse(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let _ = deserializer.into_deserializer();
} 

#[test]
fn test_map_deserializer_into_deserializer_multiple() {
    struct MultipleIterator {
        count: i32,
    }

    impl Iterator for MultipleIterator {
        type Item = (i32, bool);

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some((self.count, true))
            } else {
                None
            }
        }
    }

    let mut iterator = MultipleIterator { count: 5 };
    
    let deserializer: MapDeserializer<MultipleIterator, Error> = MapDeserializer {
        iter: iterator.fuse(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let _ = deserializer.into_deserializer();
}

