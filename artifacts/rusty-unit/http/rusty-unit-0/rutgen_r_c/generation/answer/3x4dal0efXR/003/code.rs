// Answer 0

#[test]
fn test_try_reserve_one_yellow_state_load_factor_below_threshold() {
    struct TestHeaderMap {
        entries: Vec<Bucket<HeaderValue>>,
        indices: Box<[Pos]>,
        danger: Danger,
        mask: Size,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            Self {
                entries: Vec::with_capacity(4),
                indices: Box::new([Pos::none(); 8]),
                danger: Danger::Yellow,
                mask: 7, // mask for capacity of 8
            }
        }

        fn capacity(&self) -> usize {
            usable_capacity(self.indices.len())
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn try_grow(&mut self, _new_raw_cap: usize) -> Result<(), MaxSizeReached> {
            Ok(())
        }

        fn rebuild(&mut self) {}

        fn is_yellow(&self) -> bool {
            matches!(self.danger, Danger::Yellow)
        }

        fn set_green(&mut self) {
            self.danger = Danger::Green;
        }

        fn set_red(&mut self) {
            self.danger = Danger::Red(RandomState::new());
        }
    }

    let mut map = TestHeaderMap::new();
    map.entries.push(Bucket {
        hash: HashValue(0),
        key: HeaderName::from("key1"),
        value: HeaderValue::from("value1"),
        links: None,
    });
    // This will ensure load factor is below the threshold
    map.entries.push(Bucket {
        hash: HashValue(1),
        key: HeaderName::from("key2"),
        value: HeaderValue::from("value2"),
        links: None,
    });

    assert!(map.try_reserve_one().is_ok());
}

#[test]
fn test_try_reserve_one_yellow_state_load_factor_above_threshold() {
    struct TestHeaderMap {
        entries: Vec<Bucket<HeaderValue>>,
        indices: Box<[Pos]>,
        danger: Danger,
        mask: Size,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            Self {
                entries: Vec::with_capacity(4),
                indices: Box::new([Pos::none(); 8]),
                danger: Danger::Yellow,
                mask: 7, // mask for capacity of 8
            }
        }

        fn capacity(&self) -> usize {
            usable_capacity(self.indices.len())
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn try_grow(&mut self, new_raw_cap: usize) -> Result<(), MaxSizeReached> {
            if new_raw_cap > MAX_SIZE {
                return Err(MaxSizeReached { _priv: () });
            }
            self.indices = vec![Pos::none(); new_raw_cap].into_boxed_slice();
            Ok(())
        }

        fn rebuild(&mut self) {}

        fn is_yellow(&self) -> bool {
            matches!(self.danger, Danger::Yellow)
        }
    }

    let mut map = TestHeaderMap::new();
    map.entries.push(Bucket {
        hash: HashValue(0),
        key: HeaderName::from("key1"),
        value: HeaderValue::from("value1"),
        links: None,
    });
    map.entries.push(Bucket {
        hash: HashValue(1),
        key: HeaderName::from("key2"),
        value: HeaderValue::from("value2"),
        links: None,
    });
    map.entries.push(Bucket {
        hash: HashValue(1),
        key: HeaderName::from("key3"),
        value: HeaderValue::from("value3"),
        links: None,
    });
    map.entries.push(Bucket {
        hash: HashValue(1),
        key: HeaderName::from("key4"),
        value: HeaderValue::from("value4"),
        links: None,
    });

    assert!(map.try_reserve_one().is_ok());
}

#[test]
fn test_try_reserve_one_empty_capacity() {
    struct TestHeaderMap {
        entries: Vec<Bucket<HeaderValue>>,
        indices: Box<[Pos]>,
        danger: Danger,
        mask: Size,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
                indices: Box::new([Pos::none(); 8]),
                danger: Danger::Yellow,
                mask: 7, // mask for capacity of 8
            }
        }

        fn capacity(&self) -> usize {
            usable_capacity(self.indices.len())
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn try_grow(&mut self, _new_raw_cap: usize) -> Result<(), MaxSizeReached> {
            Ok(())
        }

        fn rebuild(&mut self) {}
    }

    let mut map = TestHeaderMap::new();
    assert!(map.try_reserve_one().is_ok());
} 

#[test]
fn test_try_reserve_one_half_capacity() {
    struct TestHeaderMap {
        entries: Vec<Bucket<HeaderValue>>,
        indices: Box<[Pos]>,
        danger: Danger,
        mask: Size,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            Self {
                entries: Vec::with_capacity(4),
                indices: Box::new([Pos::none(); 8]),
                danger: Danger::Yellow,
                mask: 7, // mask for capacity of 8
            }
        }

        fn capacity(&self) -> usize {
            usable_capacity(self.indices.len())
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn try_grow(&mut self, _new_raw_cap: usize) -> Result<(), MaxSizeReached> {
            Ok(())
        }

        fn rebuild(&mut self) {}
    }

    let mut map = TestHeaderMap::new();
    map.entries.push(Bucket {
        hash: HashValue(0),
        key: HeaderName::from("key1"),
        value: HeaderValue::from("value1"),
        links: None,
    });

    assert!(map.try_reserve_one().is_ok());
}

