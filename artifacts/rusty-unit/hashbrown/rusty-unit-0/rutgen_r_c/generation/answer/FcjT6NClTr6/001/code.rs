// Answer 0

#[test]
fn test_entry_vacant() {
    use hashbrown::hash_set::{Entry, HashSet};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct CustomHasher(DefaultHasher);

    impl BuildHasher for CustomHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            self.0
        }
    }

    let mut set: HashSet<&str, CustomHasher> = HashSet::new();
    let value = "test_value";

    match set.entry(value) {
        Entry::Vacant(vacant_entry) => {
            assert!(vacant_entry.inner.key == value);
        },
        Entry::Occupied(_) => panic!("Expected a Vacant entry but found Occupied."),
    }
}

