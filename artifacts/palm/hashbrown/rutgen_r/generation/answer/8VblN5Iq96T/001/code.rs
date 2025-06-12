// Answer 0

#[test]
fn test_hasher_with_default_hash_builder() {
    use hashbrown::HashSet;
    use hashbrown::DefaultHashBuilder;

    let hasher = DefaultHashBuilder::default();
    let set: HashSet<i32> = HashSet::with_hasher(hasher);
    let retrieved_hasher: &DefaultHashBuilder = set.hasher();
    
    // Ensuring that the retrieved hasher is the same as the hasher used to create the set
    assert_eq!(retrieved_hasher as *const _, &set.hasher() as *const _);
}

#[test]
fn test_hasher_with_custom_hash_builder() {
    use hashbrown::HashSet;
    use hashbrown::BuildHasherDefault;
    use std::hash::Hasher;
    
    #[derive(Default)]
    struct CustomHasher;

    impl Hasher for CustomHasher {
        fn write(&mut self, _bytes: &[u8]) {}
        fn finish(&self) -> u64 { 0 }
    }
    
    let hasher = BuildHasherDefault::new(CustomHasher::default());
    let set: HashSet<i32, BuildHasherDefault<CustomHasher>> = HashSet::with_hasher(hasher);
    let retrieved_hasher: &BuildHasherDefault<CustomHasher> = set.hasher();

    // Ensuring that the retrieved hasher is the same as the hasher used to create the set
    assert_eq!(retrieved_hasher as *const _, &set.hasher() as *const _);
}

