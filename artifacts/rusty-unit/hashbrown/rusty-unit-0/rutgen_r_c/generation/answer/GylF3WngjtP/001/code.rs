// Answer 0

#[test]
fn test_get_many_unchecked_mut() {
    use crate::alloc::alloc::{Layout, Global};
    use core::alloc::Allocator;
    use core::ptr::NonNull;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            // Simplified allocation for testing purposes
            let ptr = core::alloc::alloc(layout);
            if ptr.is_null() {
                Err(())
            } else {
                Ok(NonNull::new_unchecked(ptr))
            }
        }
        
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            core::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    struct TestStruct {
        value: u32,
    }

    let mut raw_table: RawTable<TestStruct, TestAllocator> = RawTable::new_in(TestAllocator);
    
    // Assume we fill the raw_table with some values for the test
    let hashes = [1u64, 2u64, 3u64];
    let mut values = vec![TestStruct { value: 10 }, TestStruct { value: 20 }, TestStruct { value: 30 }];

    for i in 0..3 {
        unsafe {
            raw_table.insert(hashes[i], values[i], |v| v.value as u64);
        }
    }

    let eq = |index: usize, item: &TestStruct| {
        item.value == values[index].value
    };

    unsafe {
        let result = raw_table.get_many_unchecked_mut(hashes, eq);
        assert_eq!(result[0], Some(&mut values[0]));
        assert_eq!(result[1], Some(&mut values[1]));
        assert_eq!(result[2], Some(&mut values[2]));
    }
} 

#[test]
#[should_panic]
fn test_get_many_unchecked_mut_out_of_bounds() {
    use crate::alloc::alloc::{Layout, Global};
    use core::alloc::Allocator;
    use core::ptr::NonNull;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            let ptr = core::alloc::alloc(layout);
            if ptr.is_null() {
                Err(())
            } else {
                Ok(NonNull::new_unchecked(ptr))
            }
        }
        
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            core::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    struct TestStruct {
        value: u32,
    }

    let mut raw_table: RawTable<TestStruct, TestAllocator> = RawTable::new_in(TestAllocator);
    
    let hashes = [1u64, 2u64, 3u64];
    let valid_value = TestStruct { value: 10 };

    unsafe {
        raw_table.insert(1, valid_value, |v| v.value as u64);
    }

    let eq = |index: usize, item: &TestStruct| {
        item.value == 20 // Invalid condition to trigger panic
    };

    unsafe {
        let _result = raw_table.get_many_unchecked_mut(hashes, eq); // This should panic
    }
}

