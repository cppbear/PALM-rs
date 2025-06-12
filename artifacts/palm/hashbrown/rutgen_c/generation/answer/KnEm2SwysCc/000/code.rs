// Answer 0

#[test]
fn test_into_iter_debug_display() {
    use core::fmt::Debug;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let kv_pairs: Vec<(i32, i32)> = vec![(1, 10), (2, 20), (3, 30)];
    let raw_iter = RawIntoIter {
        iter: RawIter::new(kv_pairs.iter().map(|(k, v)| (*k, *v))),
        allocation: None,
        marker: PhantomData,
    };

    let into_iter = IntoIter {
        inner: raw_iter,
    };

    let mut output = vec![];
    {
        let result = write!(&mut output, "{:?}", into_iter);
        assert!(result.is_ok());
    }
    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("1") && output_str.contains("10")); // Test output contains the first pair
    assert!(output_str.contains("2") && output_str.contains("20")); // Test output contains the second pair
    assert!(output_str.contains("3") && output_str.contains("30")); // Test output contains the third pair
}

