// Answer 0

#[test]
fn test_occupied_error_debug() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::new_unchecked(core::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    struct DummyHashBuilder;

    impl BuildHasher for DummyHashBuilder {
        type Hasher = std::hash::rustc_hash::FxHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::rustc_hash::FxHasher::default()
        }
    }

    impl Debug for DummyHashBuilder {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "DummyHashBuilder")
        }
    }

    let mut table = HashMap::<i32, String, DummyHashBuilder, DummyAllocator>::new();
    table.insert(1, "old_value".to_string());

    let entry = OccupiedEntry {
        hash: 1,
        elem: Bucket::new((1, "old_value".to_string())),
        table: &mut table,
    };

    let error = OccupiedError {
        entry,
        value: "new_value".to_string(),
    };

    let mut output = String::new();
    let result = fmt::Formatter::write(&mut output, &error);
    assert!(result.is_ok());
    assert!(output.contains("key: 1"));
    assert!(output.contains("old_value: \"old_value\""));
    assert!(output.contains("new_value: \"new_value\""));
}

