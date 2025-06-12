// Answer 0

#[test]
fn test_hash_with_preserve_order() {
    use std::collections::HashMap;
    use std::hash::Hasher;
    use std::hash::{Hash, Hasher};

    struct TestHasher {
        state: u64,
    }

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            self.state
        }

        fn write(&mut self, bytes: &[u8]) {
            for byte in bytes {
                self.state = self.state.wrapping_add(*byte as u64);
            }
        }

        fn write_u32(&mut self, i: u32) {
            self.write(&i.to_ne_bytes());
        }

        fn write_u64(&mut self, i: u64) {
            self.write(&i.to_ne_bytes());
        }
    }

    struct TestMap {
        map: HashMap<String, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut map = HashMap::new();
            map.insert("apple".to_string(), 1);
            map.insert("banana".to_string(), 2);
            map.insert("cherry".to_string(), 3);
            TestMap { map }
        }

        fn hash<H: Hasher>(&self, state: &mut H) {
            #[cfg(not(feature = "preserve_order"))]
            {
                self.map.hash(state);
            }

            #[cfg(feature = "preserve_order")]
            {
                let mut kv: Vec<_> = self.map.iter().collect();
                kv.sort_unstable_by(|a, b| a.0.cmp(b.0));
                kv.hash(state);
            }
        }
    }

    let test_map = TestMap::new();
    let mut hasher = TestHasher { state: 0 };
    test_map.hash(&mut hasher);
    assert_eq!(hasher.finish(), 6); // This is a placeholder for expected hash value based on input.
}

#[test]
fn test_hash_without_preserve_order() {
    use std::collections::HashMap;
    use std::hash::Hasher;
    use std::hash::{Hash, Hasher};

    struct TestHasher {
        state: u64,
    }

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            self.state
        }

        fn write(&mut self, bytes: &[u8]) {
            for byte in bytes {
                self.state = self.state.wrapping_add(*byte as u64);
            }
        }

        fn write_u32(&mut self, i: u32) {
            self.write(&i.to_ne_bytes());
        }

        fn write_u64(&mut self, i: u64) {
            self.write(&i.to_ne_bytes());
        }
    }

    struct TestMap {
        map: HashMap<String, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut map = HashMap::new();
            map.insert("cat".to_string(), 1);
            map.insert("dog".to_string(), 2);
            map.insert("elephant".to_string(), 3);
            TestMap { map }
        }

        fn hash<H: Hasher>(&self, state: &mut H) {
            #[cfg(not(feature = "preserve_order"))]
            {
                self.map.hash(state);
            }

            #[cfg(feature = "preserve_order")]
            {
                let mut kv: Vec<_> = self.map.iter().collect();
                kv.sort_unstable_by(|a, b| a.0.cmp(b.0));
                kv.hash(state);
            }
        }
    }

    let test_map = TestMap::new();
    let mut hasher = TestHasher { state: 0 };
    test_map.hash(&mut hasher);
    assert_eq!(hasher.finish(), 6); // This is a placeholder for expected hash value based on input.
}

