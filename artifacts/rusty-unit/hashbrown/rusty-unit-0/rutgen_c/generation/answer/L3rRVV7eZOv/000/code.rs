// Answer 0

#[test]
fn test_raw_occupied_entry_mut_debug() {
    // Helper struct to simulate Allocator
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new(0u8))))
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
            drop(Box::from_raw(ptr.as_ptr()));
        }
    }

    // Create a dummy RawTable and Bucket
    let mut allocator = TestAllocator;
    
    let bucket = Bucket {
        ptr: NonNull::new(Box::into_raw(Box::new((42, "test".to_string())))).unwrap(),
    };

    let mut raw_table = RawTable {
        table: RawTableInner::new(), // needs an appropriate constructor
        alloc: allocator,
        marker: PhantomData,
    };

    let entry = RawOccupiedEntryMut {
        elem: bucket,
        table: &mut raw_table,
        hash_builder: &(),
    };

    // Capture output
    let mut output = vec![];
    {
        let mut formatter = fmt::Formatter::new();
        entry.fmt(&mut formatter).unwrap();
        output = formatter.into_inner();
    }

    // Assert output contains key and value
    assert!(output.contains("key: 42"));
    assert!(output.contains("value: \"test\""));
}

