// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    use std::collections::hash_map::DefaultHasher;
    use indexmap::IndexMap;
    use core::hash::BuildHasher;

    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, i32, TestHasher> = IndexMap::new();
    let hash_builder = TestHasher;

    let vacant_entry = RawEntryMut::Vacant(RawVacantEntryMut {
        map: RefMut::new(&mut map),
        hash_builder: &hash_builder,
    });

    let (key_ref, value_ref) = vacant_entry.or_insert(10, 20);
}

#[test]
fn test_or_insert_with_another_vacant_entry() {
    use std::collections::hash_map::DefaultHasher;
    use indexmap::IndexMap;
    use core::hash::BuildHasher;

    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, i32, TestHasher> = IndexMap::new();
    let hash_builder = TestHasher;

    let vacant_entry = RawEntryMut::Vacant(RawVacantEntryMut {
        map: RefMut::new(&mut map),
        hash_builder: &hash_builder,
    });

    let (key_ref, value_ref) = vacant_entry.or_insert(50, 75);
}

#[test]
fn test_or_insert_with_maximum_values() {
    use std::collections::hash_map::DefaultHasher;
    use indexmap::IndexMap;
    use core::hash::BuildHasher;

    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, i32, TestHasher> = IndexMap::new();
    let hash_builder = TestHasher;

    let vacant_entry = RawEntryMut::Vacant(RawVacantEntryMut {
        map: RefMut::new(&mut map),
        hash_builder: &hash_builder,
    });

    let (key_ref, value_ref) = vacant_entry.or_insert(100, 100);
}

#[test]
fn test_or_insert_with_minimum_values() {
    use std::collections::hash_map::DefaultHasher;
    use indexmap::IndexMap;
    use core::hash::BuildHasher;

    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, i32, TestHasher> = IndexMap::new();
    let hash_builder = TestHasher;

    let vacant_entry = RawEntryMut::Vacant(RawVacantEntryMut {
        map: RefMut::new(&mut map),
        hash_builder: &hash_builder,
    });

    let (key_ref, value_ref) = vacant_entry.or_insert(1, 1);
}

