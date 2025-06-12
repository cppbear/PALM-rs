// Answer 0

#[test]
fn test_swap_remove_full_single_entry() {
    struct SimpleHasher;
    use std::hash::{Hash, Hasher};

    impl Hasher for SimpleHasher {
        fn finish(&self) -> u64 {
            0
        }
        fn write(&mut self, bytes: &[u8]) {
            // No-op for simplicity
        }
        fn write_u64(&mut self, _: u64) {
            // No-op for simplicity
        }
    }

    impl BuildHasher for SimpleHasher {
        type Hasher = SimpleHasher;
        fn build_hasher(&self) -> Self::Hasher {
            SimpleHasher
        }
    }

    let mut map: IndexMap<i32, String, SimpleHasher> = IndexMap::new();
    map.insert(1, "one".to_string());

    let result = map.swap_remove_full(&1);
}

#[test]
fn test_swap_remove_full_empty() {
    struct SimpleHasher;
    use std::hash::{Hash, Hasher};

    impl Hasher for SimpleHasher {
        fn finish(&self) -> u64 {
            0
        }
        fn write(&mut self, bytes: &[u8]) {
            // No-op for simplicity
        }
        fn write_u64(&mut self, _: u64) {
            // No-op for simplicity
        }
    }

    impl BuildHasher for SimpleHasher {
        type Hasher = SimpleHasher;
        fn build_hasher(&self) -> Self::Hasher {
            SimpleHasher
        }
    }

    let mut map: IndexMap<i32, String, SimpleHasher> = IndexMap::new();
    let result = map.swap_remove_full(&1);
}

#[test]
fn test_swap_remove_full_multiple_entries() {
    struct SimpleHasher;
    use std::hash::{Hash, Hasher};

    impl Hasher for SimpleHasher {
        fn finish(&self) -> u64 {
            0
        }
        fn write(&mut self, bytes: &[u8]) {
            // No-op for simplicity
        }
        fn write_u64(&mut self, _: u64) {
            // No-op for simplicity
        }
    }

    impl BuildHasher for SimpleHasher {
        type Hasher = SimpleHasher;
        fn build_hasher(&self) -> Self::Hasher {
            SimpleHasher
        }
    }

    let mut map: IndexMap<i32, String, SimpleHasher> = IndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());

    let result = map.swap_remove_full(&1);
}

#[test]
fn test_swap_remove_full_key_not_found() {
    struct SimpleHasher;
    use std::hash::{Hash, Hasher};

    impl Hasher for SimpleHasher {
        fn finish(&self) -> u64 {
            0
        }
        fn write(&mut self, bytes: &[u8]) {
            // No-op for simplicity
        }
        fn write_u64(&mut self, _: u64) {
            // No-op for simplicity
        }
    }

    impl BuildHasher for SimpleHasher {
        type Hasher = SimpleHasher;
        fn build_hasher(&self) -> Self::Hasher {
            SimpleHasher
        }
    }

    let mut map: IndexMap<i32, String, SimpleHasher> = IndexMap::new();
    map.insert(1, "one".to_string());

    let result = map.swap_remove_full(&2);
}

