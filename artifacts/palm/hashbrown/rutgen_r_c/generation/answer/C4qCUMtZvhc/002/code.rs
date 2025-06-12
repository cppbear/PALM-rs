// Answer 0

#[test]
fn test_or_default_with_occupied_entry() {
    struct MockHashBuilder;
    impl BuildHasher for MockHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    struct MockAllocator;
    impl Allocator for MockAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<String, u32, MockHashBuilder, MockAllocator> = HashMap::new();
    map.insert("key".to_string(), 42);

    let entry_ref = map.entry_ref("key");
    assert_eq!(entry_ref.or_default(), &mut 42);
}

#[test]
fn test_or_default_with_vacant_entry() {
    struct MockHashBuilder;
    impl BuildHasher for MockHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    struct MockAllocator;
    impl Allocator for MockAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<String, Option<u32>, MockHashBuilder, MockAllocator> = HashMap::new();
    let entry_ref = map.entry_ref("missing_key");
    assert_eq!(entry_ref.or_default(), &mut None);
}

