// Answer 0

#[test]
fn test_get_full_mut2_found() {
    use std::collections::HashMap;
    use core::hash::BuildHasher;

    struct SimpleHasher;

    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct TestEquivalent;

    impl Equivalent<TestEquivalent> for TestEquivalent {
        fn equivalent(&self, _: &TestEquivalent) -> bool {
            true
        }
    }

    #[cfg(not(feature = "std"))]
    struct TestIndexMap {
        data: HashMap<TestEquivalent, ()>,
    }

    #[cfg(not(feature = "std"))]
    impl TestIndexMap {
        fn get_full_mut2<Q>(&mut self, _: &Q) -> Option<(usize, &mut TestEquivalent, ())>
        where
            Q: ?Sized + Hash + Equivalent<TestEquivalent>,
        {
            let index = 0;
            let value = TestEquivalent;
            Some((index, &mut value, ()))
        }
    }

    #[cfg(not(feature = "std"))]
    let mut index_set = IndexSet { map: TestIndexMap { data: HashMap::new() } };
    
    let result = index_set.get_full_mut2(&TestEquivalent);
    
    assert!(result.is_some());
    assert_eq!(result.unwrap().0, 0);
}

#[test]
fn test_get_full_mut2_not_found() {
    use std::collections::HashMap;
    use core::hash::BuildHasher;

    struct SimpleHasher;

    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct TestEquivalent;

    impl Equivalent<TestEquivalent> for TestEquivalent {
        fn equivalent(&self, _: &TestEquivalent) -> bool {
            false
        }
    }

    #[cfg(not(feature = "std"))]
    struct TestIndexMap {
        data: HashMap<TestEquivalent, ()>,
    }

    #[cfg(not(feature = "std"))]
    impl TestIndexMap {
        fn get_full_mut2<Q>(&mut self, _: &Q) -> Option<(usize, &mut TestEquivalent, ())>
        where
            Q: ?Sized + Hash + Equivalent<TestEquivalent>,
        {
            None
        }
    }

    #[cfg(not(feature = "std"))]
    let mut index_set = IndexSet { map: TestIndexMap { data: HashMap::new() } };
    
    let result = index_set.get_full_mut2(&TestEquivalent);
    
    assert!(result.is_none());
}

