// Answer 0

#[test]
fn test_from_key_with_existing_key() {
    use std::collections::hash_map::RandomState;
    use std::hash::{Hash, Hasher};
    use indexmap::{IndexMap, RawEntryMut};

    // Create a type that implements the `Equivalent` trait.
    struct MyKey(String);

    impl PartialEq for MyKey {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Hash for MyKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    // Provide a dummy equivalent trait implementation for the example.
    trait Equivalent<K> {
        fn equivalent(&self, other: &K) -> bool;
    }

    impl Equivalent<MyKey> for MyKey {
        fn equivalent(&self, other: &MyKey) -> bool {
            self.eq(other)
        }
    }

    // Create an IndexMap and insert a key-value pair.
    let mut map: IndexMap<MyKey, i32, RandomState> = IndexMap::new();
    map.insert(MyKey("key1".to_string()), 10);

    // Try accessing the entry with the existing key.
    let key = MyKey("key1".to_string());
    let entry = map.raw_entry_mut().from_key(&key);

    assert!(entry.is_some());
}

#[test]
fn test_from_key_with_non_existing_key() {
    use std::collections::hash_map::RandomState;
    use std::hash::{Hash, Hasher};
    use indexmap::{IndexMap, RawEntryMut};

    struct MyKey(String);

    impl PartialEq for MyKey {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Hash for MyKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    trait Equivalent<K> {
        fn equivalent(&self, other: &K) -> bool;
    }

    impl Equivalent<MyKey> for MyKey {
        fn equivalent(&self, other: &MyKey) -> bool {
            self.eq(other)
        }
    }

    let mut map: IndexMap<MyKey, i32, RandomState> = IndexMap::new();
    map.insert(MyKey("key1".to_string()), 10);

    // Try accessing the entry with a non-existing key.
    let key = MyKey("key2".to_string());
    let entry = map.raw_entry_mut().from_key(&key);

    assert!(entry.is_none());
}

#[test]
#[should_panic]
fn test_from_key_with_invalid_key() {
    use std::collections::hash_map::RandomState;
    use std::hash::{Hash, Hasher};
    use indexmap::{IndexMap, RawEntryMut};

    // This struct does not implement required traits correctly.
    struct InvalidKey;

    // Attempt to use an invalid key.
    let mut map: IndexMap<InvalidKey, i32, RandomState> = IndexMap::new();

    let key = InvalidKey;
    // This should panic since InvalidKey does not satisfy Hash or Equivalent traits.
    let _entry = map.raw_entry_mut().from_key(&key);
}

