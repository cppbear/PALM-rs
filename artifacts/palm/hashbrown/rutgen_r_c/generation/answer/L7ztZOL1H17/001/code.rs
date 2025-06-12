// Answer 0

#[test]
fn test_next_function() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    struct TestTable {
        values: Vec<u32>,
    }

    impl RawTable<u32, TestAllocator> {
        fn new_in_test(alloc: TestAllocator) -> Self {
            Self {
                table: RawTableInner::NEW,
                alloc,
                marker: PhantomData,
            }
        }
    }

    let mut iter = RawIter {
        iter: RawIterRange { start: 0, end: 3, current: 0 },
        items: 3,
    };

    let mut table = TestTable { values: vec![1, 2, 3] };
    let raw_table = RawTable::new_in(TestAllocator);

    let mut raw_extract_if = RawExtractIf {
        iter,
        table: &mut raw_table,
    };

    let result = raw_extract_if.next(|item| {
        *item == 2 // Constraint f(item.as_mut()) is true when item is 2.
    });

    assert_eq!(result, Some(2));
}

