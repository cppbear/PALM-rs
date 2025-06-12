// Answer 0

#[test]
fn test_union_function() {
    use hashbrown::HashSet;
    use std::hash::BuildHasherDefault;
    use std::hash::Hasher;

    struct TestHasher;

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            0
        }

        fn write(&mut self, _: &[u8]) {}

        fn write_u64(&mut self, _: u64) {}
    }

    impl BuildHasher for TestHasher {
        type Hasher = TestHasher;

        fn build_hasher(&self) -> Self::Hasher {
            TestHasher
        }
    }

    let mut set_a: HashSet<i32, BuildHasherDefault<TestHasher>> = HashSet::new();
    let mut set_b: HashSet<i32, BuildHasherDefault<TestHasher>> = HashSet::new();

    set_a.insert(1);
    set_a.insert(2);
    set_a.insert(3);

    set_b.insert(4);
    set_b.insert(2);
    set_b.insert(3);
    set_b.insert(4);
    
    // Testing the union when `self.len() <= other.len()` is false
    let union_result: HashSet<i32, BuildHasherDefault<TestHasher>> = set_a.union(&set_b).collect();

    let expected_set: HashSet<i32, BuildHasherDefault<TestHasher>> = {
        let mut s = HashSet::new();
        s.insert(1);
        s.insert(2);
        s.insert(3);
        s.insert(4);
        s
    };

    assert_eq!(union_result.len(), expected_set.len());
    for &item in expected_set.iter() {
        assert!(union_result.contains(&item));
    }
}

