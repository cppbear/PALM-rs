// Answer 0

#[test]
fn test_shift_remove_full_existing_value() {
    struct TestHasher;
    use std::hash::{Hash, Hasher};

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            0
        }

        fn write(&mut self, _: &[u8]) {}
    }

    struct TestBuildHasher;

    impl BuildHasher for TestBuildHasher {
        type Hasher = TestHasher;

        fn build_hasher(&self) -> Self::Hasher {
            TestHasher
        }
    }

    let mut index_set: super::IndexSet<i32, TestBuildHasher> = super::IndexSet {
        map: super::IndexMap {
            core: Vec::new(),
            hash_builder: TestBuildHasher,
        },
    };

    // Simulate adding some elements to index_set here
    index_set.map.core.push(1);
    index_set.map.core.push(2);
    index_set.map.core.push(3);

    // Testing removing existing value
    let result = index_set.shift_remove_full(&2);
    assert_eq!(result, Some((1, 2))); // Index of 2 is 1 and the value is 2
}

#[test]
fn test_shift_remove_full_non_existing_value() {
    struct TestHasher;
    use std::hash::{Hash, Hasher};

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            0
        }

        fn write(&mut self, _: &[u8]) {}
    }

    struct TestBuildHasher;

    impl BuildHasher for TestBuildHasher {
        type Hasher = TestHasher;

        fn build_hasher(&self) -> Self::Hasher {
            TestHasher
        }
    }

    let mut index_set: super::IndexSet<i32, TestBuildHasher> = super::IndexSet {
        map: super::IndexMap {
            core: Vec::new(),
            hash_builder: TestBuildHasher,
        },
    };

    // Simulating adding some elements to index_set here
    index_set.map.core.push(1);
    index_set.map.core.push(2);
    index_set.map.core.push(3);

    // Testing removing non-existing value
    let result = index_set.shift_remove_full(&4);
    assert_eq!(result, None);
}

#[test]
#[should_panic]
fn test_shift_remove_full_invalid_index() {
    struct TestHasher;
    use std::hash::{Hash, Hasher};

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            0
        }

        fn write(&mut self, _: &[u8]) {}
    }

    struct TestBuildHasher;

    impl BuildHasher for TestBuildHasher {
        type Hasher = TestHasher;

        fn build_hasher(&self) -> Self::Hasher {
            TestHasher
        }
    }

    let mut index_set: super::IndexSet<i32, TestBuildHasher> = super::IndexSet {
        map: super::IndexMap {
            core: Vec::new(),
            hash_builder: TestBuildHasher,
        },
    };

    // No elements to remove, expecting panic/none
    let _ = index_set.shift_remove_full(&0); 
}

