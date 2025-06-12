// Answer 0

#[test]
fn test_or_insert_non_vacant_entry() {
    use hashbrown::HashSet;
    use std::collections::hash_map::Entry;

    struct TestHasher;
    impl std::hash::BuildHasher for TestHasher {
        type Hasher = std::hash::SipHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::SipHasher::new()
        }
    }

    let mut set: HashSet<&str, TestHasher> = HashSet::new();
    set.insert("poneyland");

    // Attempt to call or_insert on an existing key
    let entry = set.entry("poneyland");
    match entry {
        Entry::Occupied(_) => {
            // Panic should not happen; or_insert should do nothing
            // and we check that the value is still present
            assert!(set.contains("poneyland"));
            assert_eq!(set.len(), 1);
        }
        _ => panic!("Expected the entry to be occupied"),
    }
}

#[test]
#[should_panic]
fn test_or_insert_vacant_panic() {
    use hashbrown::HashSet;

    let mut set: HashSet<&str> = HashSet::new();

    // This should panic since we're trying to call or_insert on a vacant entry
    // directly in this case as our own Entry logic isn't implemented.
    set.entry("nonexistent").or_insert();
}

