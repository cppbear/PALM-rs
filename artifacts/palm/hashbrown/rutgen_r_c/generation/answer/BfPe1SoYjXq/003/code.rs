// Answer 0

#[test]
fn test_raw_iter_range_new_valid_case() {
    use core::ptr::null;
    use core::alloc::alloc;
    use core::mem::size_of;

    struct TestAllocator;

    impl Allocator for TestAllocator {
        unsafe fn allocate(&self, layout: Layout) -> *mut u8 {
            alloc(layout)
        }
    }

    let allocator = TestAllocator;
    
    unsafe {
        let bucket_size = size_of::<u8>() * 4; // Allocating for 4 u8 values
        let ctrl_ptr: *const u8 = allocator.allocate(Layout::from_size_align(bucket_size, 1).unwrap());
        let bucket_ptr = NonNull::new_unchecked(allocator.allocate(Layout::from_size_align(size_of::<Bucket<u8>>(), 1).unwrap()));
        
        let bucket = Bucket { ptr: bucket_ptr };
        
        // Initialize control bytes to satisfy the conditions
        core::ptr::write(ctrl_ptr as *mut u8, 1);
        core::ptr::write(ctrl_ptr.add(1) as *mut u8, 1);
        core::ptr::write(ctrl_ptr.add(2) as *mut u8, 0); // Ensuring it's not entirely 1s
        core::ptr::write(ctrl_ptr.add(3) as *mut u8, 1);
        
        let len = 4; // Length should be a power of two

        let iter_range = RawIterRange::new(ctrl_ptr, bucket, len);
        assert!(!iter_range.current_group.is_empty());
        
        // Clean up allocations
        allocator.deallocate(ctrl_ptr as *mut u8, Layout::from_size_align(bucket_size, 1).unwrap());
        allocator.deallocate(bucket_ptr.as_ptr(), Layout::from_size_align(size_of::<Bucket<u8>>(), 1).unwrap());
    }
}

#[test]
#[should_panic]
fn test_raw_iter_range_new_invalid_pointer() {
    // Test with invalid control pointer (unaligned)
    unsafe {
        let bucket = Bucket { ptr: NonNull::new_unchecked(0x5 as *mut u8) };
        let len = 4; // Length should be a power of two
        let ctrl_ptr: *const u8 = 0x1 as *const u8; // Misaligned pointer
        RawIterRange::new(ctrl_ptr, bucket, len);
    }
}

#[test]
#[should_panic]
fn test_raw_iter_range_new_zero_length() {
    // Test with zero length, which should panic
    unsafe {
        let bucket = Bucket { ptr: NonNull::new_unchecked(0x5 as *mut u8) };
        let ctrl_ptr: *const u8 = &0x0 as *const u8; // Valid control pointer
        RawIterRange::new(ctrl_ptr, bucket, 0);
    }
}

#[test]
#[should_panic]
fn test_raw_iter_range_new_non_power_of_two_length() {
    // Test with length not being a power of two
    unsafe {
        let bucket = Bucket { ptr: NonNull::new_unchecked(0x5 as *mut u8) };
        let ctrl_ptr: *const u8 = &0x0 as *const u8; // Valid control pointer
        RawIterRange::new(ctrl_ptr, bucket, 3); // 3 is not a power of two
    }
}

