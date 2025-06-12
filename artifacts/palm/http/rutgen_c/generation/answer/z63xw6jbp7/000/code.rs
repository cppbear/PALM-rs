// Answer 0

#[test]
fn test_insert_occupied_mult() {
    struct TestHeaderMap {
        map: HeaderMap<i32>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            let capacity = 10;
            let map = HeaderMap::with_capacity(capacity);
            Self { map }
        }

        fn insert_value(&mut self, index: usize, value: i32) -> ValueDrain<i32> {
            self.map.insert_occupied_mult(index, value)
        }

        fn add_entry(&mut self, key: HeaderName, value: i32) {
            let index = self.map.entries.len();
            self.map.entries.push(Bucket {
                hash: 0, // Example hash value
                key,
                value,
                links: None,
            });
        }
    }

    let mut map = TestHeaderMap::new();
    
    let key = HeaderName::from_static("key1");
    map.add_entry(key.clone(), 10);
    
    let value_drain = map.insert_value(0, 20);
    assert_eq!(value_drain.first, Some(10));

    let key2 = HeaderName::from_static("key2");
    map.add_entry(key2.clone(), 30);
    let value_drain2 = map.insert_value(1, 40);
    assert_eq!(value_drain2.first, Some(30));
}

#[test]
fn test_insert_occupied_mult_no_entries() {
    struct TestHeaderMap {
        map: HeaderMap<i32>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            let capacity = 10;
            let map = HeaderMap::with_capacity(capacity);
            Self { map }
        }

        fn insert_value(&mut self, index: usize, value: i32) -> ValueDrain<i32> {
            self.map.insert_occupied_mult(index, value)
        }
    }

    let mut map = TestHeaderMap::new();
    let value_drain = map.insert_value(0, 15);
    assert_eq!(value_drain.first, Some(0)); // Expect 0 because no entry exists
}

