// Answer 0

#[test]
fn test_into_keys_debug_empty() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let allocator = TestAllocator;
    let empty_iter: IntoIter<i32, i32, TestAllocator> = IntoIter { inner: RawIntoIter::default() };
    let into_keys = IntoKeys { inner: empty_iter };

    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", into_keys);
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "[]");
}

#[test]
fn test_into_keys_debug_non_empty() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let allocator = TestAllocator;
    let entries = vec![(1, "one"), (2, "two")];
    let raw_iter = RawIntoIter::from_entries(entries);
    let non_empty_iter: IntoIter<i32, &str, TestAllocator> = IntoIter { inner: raw_iter };
    let into_keys = IntoKeys { inner: non_empty_iter };

    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", into_keys);
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "[1, 2]");
}

