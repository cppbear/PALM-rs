// Answer 0

#[test]
fn test_into_keys_debug_empty() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let empty_iter: IntoIter<(), (), TestAllocator> = IntoIter { 
        inner: RawIntoIter::new(Vec::new()) 
    };
    let keys = IntoKeys { inner: empty_iter };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", keys);
    
    assert!(result.is_ok());
    assert_eq!(output, "[]");
}

#[test]
fn test_into_keys_debug_single_entry() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let single_entry_iter: IntoIter<u32, &str, TestAllocator> = IntoIter { 
        inner: RawIntoIter::new(vec![(1, "one")]) 
    };
    let keys = IntoKeys { inner: single_entry_iter };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", keys);
    
    assert!(result.is_ok());
    assert_eq!(output, "[1]");
}

#[test]
fn test_into_keys_debug_multiple_entries() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let multiple_entries_iter: IntoIter<u32, &str, TestAllocator> = IntoIter { 
        inner: RawIntoIter::new(vec![(1, "one"), (2, "two"), (3, "three")]) 
    };
    let keys = IntoKeys { inner: multiple_entries_iter };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", keys);
    
    assert!(result.is_ok());
    assert_eq!(output, "[1, 2, 3]");
}

