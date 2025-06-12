// Answer 0

#[test]
fn test_or_default_vacant_entry() {
    use hashbrown::HashMap;

    struct TestHasher;

    impl std::hash::Hasher for TestHasher {
        fn write(&mut self, bytes: &[u8]) {
            // No-op
        }

        fn finish(&self) -> u64 {
            0 // Dummy value
        }
    }

    struct TestBuildHasher;

    impl Default for TestBuildHasher {
        fn default() -> Self {
            TestBuildHasher
        }
    }

    impl hashbrown::BuildHasher for TestBuildHasher {
        type Hasher = TestHasher;

        fn build_hasher(&self) -> Self::Hasher {
            TestHasher
        }
    }

    let mut map: HashMap<String, Option<u32, TestBuildHasher>> = HashMap::default();
    assert_eq!(map.entry_ref("poneyland").or_default(), &mut None);
}

#[test]
fn test_or_default_occupied_entry() {
    use hashbrown::HashMap;

    struct TestHasher;

    impl std::hash::Hasher for TestHasher {
        fn write(&mut self, bytes: &[u8]) {
            // No-op
        }

        fn finish(&self) -> u64 {
            0 // Dummy value
        }
    }

    struct TestBuildHasher;

    impl Default for TestBuildHasher {
        fn default() -> Self {
            TestBuildHasher
        }
    }

    impl hashbrown::BuildHasher for TestBuildHasher {
        type Hasher = TestHasher;

        fn build_hasher(&self) -> Self::Hasher {
            TestHasher
        }
    }

    let mut map: HashMap<String, Option<u32, TestBuildHasher>> = HashMap::default();
    map.insert("horseland".to_string(), Some(3));
    assert_eq!(map.entry_ref("horseland").or_default(), &mut Some(3));
}

