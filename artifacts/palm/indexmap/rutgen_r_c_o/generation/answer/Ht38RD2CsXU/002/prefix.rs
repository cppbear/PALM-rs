// Answer 0

#[test]
fn test_from_hash_occupied_entry() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct MyKey {
        value: usize,
    }

    impl Hash for MyKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }

    struct MyEquivalent;

    impl Equivalent<MyKey> for MyEquivalent {
        fn equivalent(&self, _: &MyKey) -> bool {
            false
        }
    }

    let mut map = IndexMap::new();
    for i in 0..100 {
        map.insert(MyKey { value: i }, i as u32);
    }

    let mut hasher = DefaultHasher::new();
    let hash = 1; // Assuming this gives an entry present in the map.
    let builder = RawEntryBuilderMut { map: &mut map };

    let _entry = builder.from_hash(hash, |key: &MyKey| key.value == 1);
}

#[test]
fn test_from_hash_vacant_entry() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct MyKey {
        value: usize,
    }

    impl Hash for MyKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }

    struct MyEquivalent;

    impl Equivalent<MyKey> for MyEquivalent {
        fn equivalent(&self, _: &MyKey) -> bool {
            false
        }
    }

    let mut map = IndexMap::new();
    for i in 0..10 {
        map.insert(MyKey { value: i }, i as u32);
    }

    let mut hasher = DefaultHasher::new();
    let hash = 12345; // Assuming this gives an entry not present in the map.
    let builder = RawEntryBuilderMut { map: &mut map };

    let _entry = builder.from_hash(hash, |key: &MyKey| key.value == 100);
}

