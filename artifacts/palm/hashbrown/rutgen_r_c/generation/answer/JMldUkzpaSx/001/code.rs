// Answer 0

#[test]
fn test_entry_vacant() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct CustomHasher(DefaultHasher);

    impl BuildHasher for CustomHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::new_unchecked(core::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let mut map: HashMap<char, usize, CustomHasher, TestAllocator> = HashMap {
        hash_builder: CustomHasher(DefaultHasher::new()),
        table: RawTable { table: RawTableInner::new(), alloc: TestAllocator, marker: PhantomData },
    };

    let key = 'a';
    let entry = map.entry(key);

    if let Entry::Vacant(vacant_entry) = entry {
        assert_eq!(vacant_entry.key, key);
        assert!(vacant_entry.table as *mut _ == &mut map as *mut _);
    } else {
        panic!("Expected entry to be Vacant");
    }
}

