// Answer 0

#[test]
fn test_erase_valid_bucket() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            // Dummy allocation logic for testing
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable::<i32, TestAllocator>::new_in(TestAllocator);
    let bucket = Bucket {
        ptr: NonNull::from(&mut 42),
    };

    unsafe {
        table.erase(bucket);
    }
}

#[test]
#[should_panic(expected = "drop panicked")]
fn test_erase_panicking_drop() {
    struct PanickingAllocator;

    unsafe impl Allocator for PanickingAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    struct PanicOnDrop;

    impl Drop for PanicOnDrop {
        fn drop(&mut self) {
            panic!("drop panicked");
        }
    }

    let mut table = RawTable::<PanicOnDrop, PanickingAllocator>::new_in(PanickingAllocator);
    let bucket = Bucket {
        ptr: NonNull::from(Box::leak(Box::new(PanicOnDrop))),
    };

    unsafe {
        table.erase(bucket);
    }
}

