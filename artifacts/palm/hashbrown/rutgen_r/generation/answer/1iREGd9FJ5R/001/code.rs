// Answer 0

#[test]
fn test_insert_with_vacant_entry() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    use std::hash::Hasher;

    struct DummyHasher;
    impl Hasher for DummyHasher {
        fn finish(&self) -> u64 { 0 }
        fn write(&mut self, _: &[u8]) { }
    }

    struct DummyHasherBuilder;
    impl BuildHasher for DummyHasherBuilder {
        type Hasher = DummyHasher;
        fn build_hasher(&self) -> Self::Hasher {
            DummyHasher
        }
    }

    let mut map: HashMap<&str, u32, DummyHasherBuilder> = HashMap::with_capacity_and_hasher(1, DummyHasherBuilder);
    let entry = map.raw_entry_mut().from_key("vacantland").insert("vacantland", 25);
    
    assert_eq!(entry.remove_entry(), ("vacantland", 25));
}

#[test]
fn test_insert_multiple_vacant_entries() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    use std::hash::Hasher;

    struct DummyHasher;
    impl Hasher for DummyHasher {
        fn finish(&self) -> u64 { 0 }
        fn write(&mut self, _: &[u8]) { }
    }

    struct DummyHasherBuilder;
    impl BuildHasher for DummyHasherBuilder {
        type Hasher = DummyHasher;
        fn build_hasher(&self) -> Self::Hasher {
            DummyHasher
        }
    }

    let mut map: HashMap<&str, u32, DummyHasherBuilder> = HashMap::with_capacity_and_hasher(2, DummyHasherBuilder);
    let entry1 = map.raw_entry_mut().from_key("first").insert("first", 1);
    let entry2 = map.raw_entry_mut().from_key("second").insert("second", 2);
    
    assert_eq!(entry1.remove_entry(), ("first", 1));
    assert_eq!(entry2.remove_entry(), ("second", 2));
}

#[test]
#[should_panic]
fn test_insert_panic_on_duplicate_key() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    use std::hash::Hasher;

    struct DummyHasher;
    impl Hasher for DummyHasher {
        fn finish(&self) -> u64 { 0 }
        fn write(&mut self, _: &[u8]) { }
    }

    struct DummyHasherBuilder;
    impl BuildHasher for DummyHasherBuilder {
        type Hasher = DummyHasher;
        fn build_hasher(&self) -> Self::Hasher {
            DummyHasher
        }
    }

    let mut map: HashMap<&str, u32, DummyHasherBuilder> = HashMap::with_capacity_and_hasher(1, DummyHasherBuilder);
    map.raw_entry_mut().from_key("duplicate").insert("duplicate", 10);
    let _entry = map.raw_entry_mut().from_key("duplicate").insert("duplicate", 20); // This should panic
}

