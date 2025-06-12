// Answer 0

#[test]
fn test_data_end() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            NonNull::new_unchecked(Box::into_raw(Box::new([0u8; 64])) as *mut u8)
                .ok_or(())
        }
        
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            drop(Box::from_raw(ptr.as_ptr() as *mut [u8; 64]));
        }
    }

    let allocator = MockAllocator;
    let raw_table: RawTable<i32, MockAllocator> = RawTable::new_in(allocator);
    
    let data_end_ptr = raw_table.data_end();
    assert!(!data_end_ptr.as_ptr().is_null());
}

#[test]
fn test_data_end_with_capacity() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            NonNull::new_unchecked(Box::into_raw(Box::new([0u8; 64])) as *mut u8)
                .ok_or(())
        }
        
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            drop(Box::from_raw(ptr.as_ptr() as *mut [u8; 64]));
        }
    }

    let allocator = MockAllocator;
    let mut raw_table: RawTable<i32, MockAllocator> = RawTable::with_capacity_in(2, allocator);
    
    let data_end_ptr = raw_table.data_end();
    assert!(!data_end_ptr.as_ptr().is_null());
}

