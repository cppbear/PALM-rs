// Answer 0

#[test]
fn test_into_allocation_non_empty() {
    struct TestAllocator;

    impl TestAllocator {
        fn new() -> Self {
            TestAllocator
        }
    }

    struct TestTable {
        buckets: usize,
    }

    impl TestTable {
        fn is_empty_singleton(&self) -> bool {
            self.buckets == 0
        }

        fn buckets(&self) -> usize {
            self.buckets
        }
    }

    struct TestStruct {
        table: TestTable,
        alloc: TestAllocator,
    }

    impl TestStruct {
        fn new(buckets: usize) -> Self {
            TestStruct {
                table: TestTable { buckets },
                alloc: TestAllocator::new(),
            }
        }

        fn into_allocation(self) -> Option<(std::ptr::NonNull<u8>, std::alloc::Layout, TestAllocator)> {
            let alloc = if self.table.is_empty_singleton() {
                None
            } else {
                let ctrl_offset = std::mem::size_of::<TestAllocator>();
                let layout = std::alloc::Layout::from_size_align(std::mem::size_of::<TestAllocator>(), std::mem::align_of::<TestAllocator>()).unwrap();
                Some((
                    unsafe { std::ptr::NonNull::new_unchecked(self.alloc as *const _ as *mut u8) },
                    layout,
                    unsafe { std::ptr::read(&self.alloc) },
                ))
            };
            std::mem::forget(self);
            alloc
        }
    }

    let test_struct = TestStruct::new(1);
    let allocation = test_struct.into_allocation();
    assert!(allocation.is_some());
}

#[test]
fn test_into_allocation_empty() {
    struct TestAllocator;

    impl TestAllocator {
        fn new() -> Self {
            TestAllocator
        }
    }

    struct TestTable {
        buckets: usize,
    }

    impl TestTable {
        fn is_empty_singleton(&self) -> bool {
            self.buckets == 0
        }

        fn buckets(&self) -> usize {
            self.buckets
        }
    }

    struct TestStruct {
        table: TestTable,
        alloc: TestAllocator,
    }

    impl TestStruct {
        fn new(buckets: usize) -> Self {
            TestStruct {
                table: TestTable { buckets },
                alloc: TestAllocator::new(),
            }
        }

        fn into_allocation(self) -> Option<(std::ptr::NonNull<u8>, std::alloc::Layout, TestAllocator)> {
            let alloc = if self.table.is_empty_singleton() {
                None
            } else {
                let ctrl_offset = std::mem::size_of::<TestAllocator>();
                let layout = std::alloc::Layout::from_size_align(std::mem::size_of::<TestAllocator>(), std::mem::align_of::<TestAllocator>()).unwrap();
                Some((
                    unsafe { std::ptr::NonNull::new_unchecked(self.alloc as *const _ as *mut u8) },
                    layout,
                    unsafe { std::ptr::read(&self.alloc) },
                ))
            };
            std::mem::forget(self);
            alloc
        }
    }

    let test_struct = TestStruct::new(0);
    let allocation = test_struct.into_allocation();
    assert!(allocation.is_none());
}

