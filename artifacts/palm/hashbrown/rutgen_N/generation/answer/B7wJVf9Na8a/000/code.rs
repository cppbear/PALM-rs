// Answer 0

#[test]
fn test_insert_entry() {
    use std::hash::{Hash, Hasher};
    use hashbrown::HashMap;

    struct SimpleHasher {
        value: usize,
    }

    impl Default for SimpleHasher {
        fn default() -> Self {
            SimpleHasher { value: 0 }
        }
    }

    impl Hasher for SimpleHasher {
        fn finish(&self) -> u64 {
            self.value as u64
        }

        fn write(&mut self, bytes: &[u8]) {
            self.value += bytes.len();
        }
    }

    struct CustomHashBuilder {
        hasher: SimpleHasher,
    }

    impl BuildHasher for CustomHashBuilder {
        type Hasher = SimpleHasher;

        fn build_hasher(&self) -> Self::Hasher {
            SimpleHasher::default()
        }
    }

    struct TestEntry {
        hash_builder: CustomHashBuilder,
        table: HashMap<String, i32, CustomHashBuilder>,
    }

    impl TestEntry {
        fn new() -> Self {
            TestEntry {
                hash_builder: CustomHashBuilder { hasher: SimpleHasher::default() },
                table: HashMap::with_hasher(CustomHashBuilder { hasher: SimpleHasher::default() }),
            }
        }

        fn insert_entry(self, key: String, value: i32) -> (String, i32) {
            let hash = self.hash_builder.build_hasher().finish() as usize; // Simplified hash for testing
            self.table.insert(key.clone(), value);
            (key, value)
        }
    }

    let test_entry = TestEntry::new();
    let (key, value) = test_entry.insert_entry(String::from("test_key"), 42);
    assert_eq!(value, 42);
    assert_eq!(test_entry.table.get(&key).unwrap(), &42);
}

#[test]
fn test_insert_empty_key() {
    use std::hash::{Hash, Hasher};
    use hashbrown::HashMap;

    struct SimpleHasher {
        value: usize,
    }

    impl Default for SimpleHasher {
        fn default() -> Self {
            SimpleHasher { value: 0 }
        }
    }

    impl Hasher for SimpleHasher {
        fn finish(&self) -> u64 {
            self.value as u64
        }

        fn write(&mut self, bytes: &[u8]) {
            self.value += bytes.len();
        }
    }

    struct CustomHashBuilder {
        hasher: SimpleHasher,
    }

    impl BuildHasher for CustomHashBuilder {
        type Hasher = SimpleHasher;

        fn build_hasher(&self) -> Self::Hasher {
            SimpleHasher::default()
        }
    }

    struct TestEntry {
        hash_builder: CustomHashBuilder,
        table: HashMap<String, i32, CustomHashBuilder>,
    }

    impl TestEntry {
        fn new() -> Self {
            TestEntry {
                hash_builder: CustomHashBuilder { hasher: SimpleHasher::default() },
                table: HashMap::with_hasher(CustomHashBuilder { hasher: SimpleHasher::default() }),
            }
        }

        fn insert_entry(self, key: String, value: i32) -> (String, i32) {
            let hash = self.hash_builder.build_hasher().finish() as usize; // Simplified hash for testing
            self.table.insert(key.clone(), value);
            (key, value)
        }
    }

    let test_entry = TestEntry::new();
    let (key, value) = test_entry.insert_entry(String::from(""), 0);
    assert_eq!(value, 0);
    assert_eq!(test_entry.table.get(&key).unwrap(), &0);
}

