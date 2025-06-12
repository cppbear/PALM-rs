// Answer 0

#[test]
fn test_try_reserve_success() {
    struct TestHeaderMap {
        map: HeaderMap<HeaderValue>,
    }

    impl TestHeaderMap {
        fn new(initial_capacity: usize) -> Self {
            let mut map = HeaderMap::with_capacity(initial_capacity);
            for i in 0..initial_capacity {
                map.insert(format!("key-{}", i), HeaderValue::from(format!("value-{}", i))).unwrap();
            }
            Self { map }
        }

        fn try_reserve(&mut self, additional: usize) -> Result<(), MaxSizeReached> {
            self.map.try_reserve(additional)
        }
    }

    let mut test_map = TestHeaderMap::new(8);
    let result = test_map.try_reserve(4);
    assert!(result.is_ok());
}

#[test]
fn test_try_reserve_exceeds_limit() {
    struct TestHeaderMap {
        map: HeaderMap<HeaderValue>,
    }

    impl TestHeaderMap {
        fn new(initial_capacity: usize) -> Self {
            let mut map = HeaderMap::with_capacity(initial_capacity);
            for i in 0..initial_capacity {
                map.insert(format!("key-{}", i), HeaderValue::from(format!("value-{}", i))).unwrap();
            }
            Self { map }
        }

        fn try_reserve(&mut self, additional: usize) -> Result<(), MaxSizeReached> {
            self.map.try_reserve(additional)
        }
    }

    let mut test_map = TestHeaderMap::new(8);
    // Make sure the next power of two exceeds the max size
    let result = test_map.try_reserve(65530); // This should cause the allocation to exceed MAX_SIZE
    assert!(result.is_err());
}

#[test]
fn test_try_reserve_grow_needed() {
    struct TestHeaderMap {
        map: HeaderMap<HeaderValue>,
    }

    impl TestHeaderMap {
        fn new(initial_capacity: usize) -> Self {
            let mut map = HeaderMap::with_capacity(initial_capacity);
            for i in 0..initial_capacity {
                map.insert(format!("key-{}", i), HeaderValue::from(format!("value-{}", i))).unwrap();
            }
            Self { map }
        }

        fn try_reserve(&mut self, additional: usize) -> Result<(), MaxSizeReached> {
            self.map.try_reserve(additional)
        }
    }

    let mut test_map = TestHeaderMap::new(8);
    let initial_capacity = 8;
    let additional_size = 10; // Just to trigger the grow phase
    let result = test_map.try_reserve(additional_size);
    assert!(result.is_ok());
}

