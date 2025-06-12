// Answer 0

#[test]
fn test_debug_fmt_non_empty() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let entries: Vec<(&str, &str)> = vec![("key1", "value1"), ("key2", "value2")];
    let iter = entries.iter();
    let drain = Drain { iter };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", drain);
    assert!(result.is_ok());
    assert_eq!(buffer, "[key1, key2]");
}

#[test]
#[should_panic]
fn test_debug_fmt_empty() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let entries: Vec<(&str, &str)> = vec![];
    let iter = entries.iter();
    let drain = Drain { iter };
    let mut buffer = String::new();
    write!(&mut buffer, "{:?}", drain).unwrap();
}

