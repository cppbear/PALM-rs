// Answer 0

#[test]
fn test_insert_full_new_value() {
    struct TestHasher;
    use std::hash::{Hasher, Hash};

    impl BuildHasher for TestHasher {
        type Hasher = TestHasher;

        fn build_hasher(&self) -> Self::Hasher {
            TestHasher
        }
    }

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            0
        }

        fn write(&mut self, _: &[u8]) {}

        fn write_u8(&mut self, _: u8) {}
    }

    let mut index_set: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore { /* initialize as needed */ },
            hash_builder: TestHasher,
        },
    };

    let (index, inserted) = index_set.insert_full(10);
    assert_eq!(index, 0);
    assert!(inserted);
}

#[test]
fn test_insert_full_existing_value() {
    struct TestHasher;
    use std::hash::{Hasher, Hash};

    impl BuildHasher for TestHasher {
        type Hasher = TestHasher;

        fn build_hasher(&self) -> Self::Hasher {
            TestHasher
        }
    }

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            0
        }

        fn write(&mut self, _: &[u8]) {}

        fn write_u8(&mut self, _: u8) {}
    }

    let mut index_set: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore { /* initialize as needed */ },
            hash_builder: TestHasher,
        },
    };

    index_set.insert_full(10);
    let (index, inserted) = index_set.insert_full(10);
    assert_eq!(index, 0);
    assert!(!inserted);
}

