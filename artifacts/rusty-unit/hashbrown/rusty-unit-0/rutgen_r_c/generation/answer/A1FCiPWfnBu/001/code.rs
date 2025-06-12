// Answer 0

#[test]
fn test_into_values_debug_fmt_empty() {
    struct TestAllocator {}

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let values: IntoValues<i32, &str, TestAllocator> = IntoValues {
        inner: IntoIter {
            inner: RawIntoIter::new(vec![].into_iter(), TestAllocator {}),
        },
    };

    let mut output = vec![];
    let result = write!(&mut output, "{:?}", values);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), "[]");
}

#[test]
fn test_into_values_debug_fmt_single_entry() {
    struct TestAllocator {}

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let values: IntoValues<i32, &str, TestAllocator> = IntoValues {
        inner: IntoIter {
            inner: RawIntoIter::new(vec![(1, "one")].into_iter(), TestAllocator {}),
        },
    };

    let mut output = vec![];
    let result = write!(&mut output, "{:?}", values);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), "[\"one\"]");
}

#[test]
fn test_into_values_debug_fmt_multiple_entries() {
    struct TestAllocator {}

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let values: IntoValues<i32, &str, TestAllocator> = IntoValues {
        inner: IntoIter {
            inner: RawIntoIter::new(vec![(1, "one"), (2, "two"), (3, "three")].into_iter(), TestAllocator {}),
        },
    };

    let mut output = vec![];
    let result = write!(&mut output, "{:?}", values);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), "[\"one\", \"two\", \"three\"]");
}

#[test]
#[should_panic]
fn test_into_values_debug_fmt_panic() {
    struct PanicAllocator {}

    unsafe impl Allocator for PanicAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            panic!("Allocation failed")
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let values: IntoValues<i32, &str, PanicAllocator> = IntoValues {
        inner: IntoIter {
            inner: RawIntoIter::new(vec![(1, "one")].into_iter(), PanicAllocator {}),
        },
    };

    let mut output = vec![];
    let _ = write!(&mut output, "{:?}", values);
}

