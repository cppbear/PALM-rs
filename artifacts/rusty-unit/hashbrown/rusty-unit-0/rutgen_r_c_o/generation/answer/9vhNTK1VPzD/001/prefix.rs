// Answer 0

#[test]
fn test_with_hasher_in_default_hash_builder() {
    let alloc = Global;
    let hash_builder = DefaultHashBuilder::default();
    let map = HashMap::with_hasher_in(hash_builder, alloc);
}

#[test]
fn test_with_hasher_in_custom_allocator() {
    struct CustomAllocator;
    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = CustomAllocator;
    let hash_builder = DefaultHashBuilder::default();
    let map = HashMap::with_hasher_in(hash_builder, alloc);
}

#[test]
fn test_with_hasher_in_zero_capacity() {
    let alloc = Global;
    let hash_builder = DefaultHashBuilder::default();
    let map = HashMap::with_hasher_in(hash_builder, alloc);
}

#[test]
fn test_with_hasher_in_non_default_hash_builder() {
    struct DummyHashBuilder;
    impl BuildHasher for DummyHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let alloc = Global;
    let hash_builder = DummyHashBuilder;
    let map = HashMap::with_hasher_in(hash_builder, alloc);
}

#[should_panic]
fn test_with_hasher_in_invalid_allocator() {
    struct InvalidAllocator;
    unsafe impl Allocator for InvalidAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = InvalidAllocator;
    let hash_builder = DefaultHashBuilder::default();
    let map = HashMap::with_hasher_in(hash_builder, alloc);
}

