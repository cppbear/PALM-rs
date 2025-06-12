// Answer 0

#[test]
fn test_debug_fmt_for_drain() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    struct TestDrain<'a> {
        iter: std::iter::Iterator<Item = (&'a str, ())>,
    }

    impl<'a> fmt::Debug for TestDrain<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let entries_iter = self.iter.map(|(k, _)| k);
            f.debug_list().entries(entries_iter).finish()
        }
    }

    let data = vec![("key1", ()), ("key2", ()), ("key3", ())];
    let drain = TestDrain { iter: data.iter().map(|(k, v)| (*k, *v)) };

    let mut formatted = String::new();
    let result = write!(&mut formatted, "{:?}", drain);
    assert!(result.is_ok());
    assert_eq!(formatted, "[\"key1\", \"key2\", \"key3\"]");
}

