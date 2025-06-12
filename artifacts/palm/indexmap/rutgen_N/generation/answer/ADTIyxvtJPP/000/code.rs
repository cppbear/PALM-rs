// Answer 0

#[test]
fn test_hasher() {
    use std::collections::HashMap;
    use std::hash::BuildHasher;

    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct MySet<S: BuildHasher> {
        map: HashMap<i32, i32, S>,
    }

    impl<S: BuildHasher> MySet<S> {
        pub fn new(hasher: S) -> Self {
            MySet {
                map: HashMap::with_hasher(hasher),
            }
        }

        pub fn hasher(&self) -> &S {
            // Creating a reference to the hasher from the map
            // This is simplified; the actual implementation would store `S`
            &self.map.hasher()
        }
    }

    let hasher = SimpleHasher;
    let set = MySet::new(hasher);
        
    let retrieved_hasher = set.hasher();
    
    assert!(std::ptr::eq(retrieved_hasher as *const _, set.map.hasher() as *const _));
}

