// Answer 0

#[test]
fn test_raw_iter_range_new_valid_case() {
    use core::alloc::Layout;
    use core::ptr::NonNull;
    
    struct TestBucket<T> {
        ptr: NonNull<T>,
    }
    
    // Create an instance of Bucket with a valid pointer
    let layout = Layout::from_size_align(32, 16).unwrap(); // Adjust size and alignment as necessary
    let ptr = unsafe { NonNull::new_unchecked(core::alloc::alloc(layout)) };
    let bucket = TestBucket { ptr };
    
    // Create a control pointer aligned to Group::WIDTH
    let ctrl = bucket.ptr.as_ptr() as *const u8; // Assume this is valid and aligned
    let len = 32; // Ensure len is a power of two

    unsafe {
        let raw_iter_range = RawIterRange::new(ctrl, bucket, len);
        assert!(!raw_iter_range.current_group.0.0.is_null());
    }
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_raw_iter_range_new_zero_length() {
    use core::alloc::Layout;
    use core::ptr::NonNull;
    
    struct TestBucket<T> {
        ptr: NonNull<T>,
    }

    let layout = Layout::from_size_align(32, 16).unwrap(); // Adjust size and alignment as necessary
    let ptr = unsafe { NonNull::new_unchecked(core::alloc::alloc(layout)) };
    let bucket = TestBucket { ptr };
    
    // Create a control pointer aligned to Group::WIDTH
    let ctrl = bucket.ptr.as_ptr() as *const u8; // Assume this is valid and aligned
    let len = 0; // This should panic

    unsafe {
        RawIterRange::new(ctrl, bucket, len);
    }
}

#[test]
#[should_panic]
fn test_raw_iter_range_new_invalid_alignment() {
    use core::alloc::Layout;
    use core::ptr::NonNull;

    struct TestBucket<T> {
        ptr: NonNull<T>,
    }

    let layout = Layout::from_size_align(32, 16).unwrap(); 
    let ptr = unsafe { NonNull::new_unchecked(core::alloc::alloc(layout)) };
    let bucket = TestBucket { ptr };

    // Control pointer not aligned to Group::WIDTH (intentionally misaligned)
    let ctrl = (bucket.ptr.as_ptr() as usize + 1) as *const u8; 
    let len = 32; // A valid power of two

    unsafe {
        RawIterRange::new(ctrl, bucket, len);
    }
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_raw_iter_range_new_len_exceeds_buckets() {
    use core::alloc::Layout;
    use core::ptr::NonNull;

    struct TestBucket<T> {
        ptr: NonNull<T>,
    }

    let layout = Layout::from_size_align(32, 16).unwrap(); 
    let ptr = unsafe { NonNull::new_unchecked(core::alloc::alloc(layout)) };
    let bucket = TestBucket { ptr };

    let ctrl = bucket.ptr.as_ptr() as *const u8; 
    let len = 64; // More than the number of table buckets

    unsafe {
        RawIterRange::new(ctrl, bucket, len);
    }
}

