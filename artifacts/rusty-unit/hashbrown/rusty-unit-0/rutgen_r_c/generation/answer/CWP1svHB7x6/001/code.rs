// Answer 0

#[test]
fn test_fmt_with_empty_iter() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let empty_iter = IntoIter {
        inner: RawIntoIter::empty(),
    };

    let result = std::panic::catch_unwind(|| {
        let mut vec = Vec::new();
        write!(&mut vec, "{:?}", empty_iter).unwrap();
        String::from_utf8(vec).unwrap()
    });

    assert!(result.is_ok());
}

#[test]
fn test_fmt_with_single_element_iter() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let single_iter = IntoIter {
        inner: RawIntoIter::from(vec![(1, "a")]),
    };

    let result = std::panic::catch_unwind(|| {
        let mut vec = Vec::new();
        write!(&mut vec, "{:?}", single_iter).unwrap();
        String::from_utf8(vec).unwrap()
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "[1]");
}

#[test]
fn test_fmt_with_multiple_elements_iter() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let multi_iter = IntoIter {
        inner: RawIntoIter::from(vec![(1, "a"), (2, "b"), (3, "c")]),
    };

    let result = std::panic::catch_unwind(|| {
        let mut vec = Vec::new();
        write!(&mut vec, "{:?}", multi_iter).unwrap();
        String::from_utf8(vec).unwrap()
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "[1, 2, 3]");
}

