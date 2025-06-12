// Answer 0

#[test]
fn test_bucket_index() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            NonNull::new_unchecked(0 as *mut u8).ok_or(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = MockAllocator;
    let mut raw_table: RawTable<u32, MockAllocator> = RawTable::new_in(allocator);
    
    // Simulate the internal state correctly to create a Bucket
    let bucket_ptr = NonNull::new_unchecked(0x1000 as *mut u32); // Arbitrary address
    let bucket = Bucket { ptr: bucket_ptr };
    
    let index = unsafe { raw_table.bucket_index(&bucket) };
    assert_eq!(index, 0); // Since we set a fixed bucket pointer, expected index is 0
}

