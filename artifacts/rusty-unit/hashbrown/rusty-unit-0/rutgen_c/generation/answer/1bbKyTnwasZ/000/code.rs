// Answer 0

#[test]
fn test_find_or_find_insert_slot_existing_key() {
    use std::hash::{Hash, Hasher};
    struct SimpleHasher {
        hash: u64,
    }

    impl Default for SimpleHasher {
        fn default() -> Self {
            Self { hash: 0 }
        }
    }

    impl Hasher for SimpleHasher {
        fn finish(&self) -> u64 {
            self.hash
        }

        fn write(&mut self, _: &[u8]) {
            self.hash = 42; // dummy hash value for the sake of testing
        }

        fn write_u64(&mut self, i: u64) {
            self.hash = i;
        }
    }

    struct MyKey(u32);
    impl Hash for MyKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl Equivalent<MyKey> for MyKey {
        fn equivalent(&self, other: &MyKey) -> bool {
            self.0 == other.0
        }
    }

    let mut hashmap = HashMap::<MyKey, String, SimpleHasher>::default();
    let key = MyKey(1);
    hashmap.insert(key.clone(), "value1".to_string());
    let hash = 42;

    let result = hashmap.find_or_find_insert_slot(hash, &key).unwrap();

    assert_eq!(result.ptr.as_ref().0, (key, "value1".to_string()));
}

#[test]
fn test_find_or_find_insert_slot_new_key() {
    use std::hash::{Hash, Hasher};

    struct SimpleHasher {
        hash: u64,
    }

    impl Default for SimpleHasher {
        fn default() -> Self {
            Self { hash: 0 }
        }
    }

    impl Hasher for SimpleHasher {
        fn finish(&self) -> u64 {
            self.hash
        }

        fn write(&mut self, _: &[u8]) {
            self.hash = 42; // dummy hash value for the sake of testing
        }

        fn write_u64(&mut self, i: u64) {
            self.hash = i;
        }
    }

    struct MyKey(u32);
    impl Hash for MyKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl Equivalent<MyKey> for MyKey {
        fn equivalent(&self, other: &MyKey) -> bool {
            self.0 == other.0
        }
    }

    let mut hashmap = HashMap::<MyKey, String, SimpleHasher>::default();
    let key = MyKey(2);
    let hash = 42;

    let result = hashmap.find_or_find_insert_slot(hash, &key).unwrap_err();
    
    assert!(matches!(result, crate::raw::InsertSlot { .. }));
}

