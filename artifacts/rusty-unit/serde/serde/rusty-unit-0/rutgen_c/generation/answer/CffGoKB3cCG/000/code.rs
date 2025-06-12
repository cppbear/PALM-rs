// Answer 0

#[test]
fn test_map_deserializer_into_deserializer() {
    use std::marker::PhantomData;

    struct TestIterator {
        count: usize,
    }

    impl Iterator for TestIterator {
        type Item = (i32, i32); // Simulate a Tuple representing a key-value pair

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some((self.count as i32, self.count as i32 * 2))
            } else {
                None
            }
        }
    }

    struct TestError;
    impl de::Error for TestError {}

    let iter = TestIterator { count: 0 };
    let deserializer: MapDeserializer<_, TestError> = MapDeserializer {
        iter: iter.fuse(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let result = deserializer.into_deserializer();
    assert_eq!(result.iter.count, 0);
}

#[test]
fn test_unit_deserializer_into_deserializer() {
    struct Unit;

    let unit_deserializer: UnitDeserializer<TestError> = UnitDeserializer::new(true);
    let result = unit_deserializer.into_deserializer();
    assert_eq!(result.value, true);
}

