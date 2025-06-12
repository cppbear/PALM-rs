// Answer 0

#[test]
fn test_raw_iter_range_new_valid() {
    use core::alloc::{GlobalAlloc, Layout};
    use std::alloc::{alloc, dealloc};

    struct MyAllocator;

    unsafe impl GlobalAlloc for MyAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            alloc(layout)
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            dealloc(ptr, layout)
        }
    }

    let allocator = MyAllocator;
    let layout = Layout::from_size_align(64, 8).unwrap();
    let ctrl_ptr = unsafe { allocator.alloc(layout) };
    let data_bucket = Bucket {
        ptr: NonNull::new(ctrl_ptr).unwrap(),
    };
    
    let len: usize = 8; // must be a power of two
    let aligned_ctrl = ctrl_ptr as *const u8;

    let _iter_range: RawIterRange<u8> = unsafe { RawIterRange::new(aligned_ctrl, data_bucket, len) };

    unsafe {
        allocator.dealloc(ctrl_ptr, layout);
    }
}

#[test]
#[should_panic]
fn test_raw_iter_range_new_zero_length() {
    let layout = Layout::from_size_align(64, 8).unwrap();
    let ctrl_ptr = unsafe { std::alloc::alloc(layout) };
    
    let data_bucket = Bucket {
        ptr: NonNull::new(ctrl_ptr).unwrap(),
    };
    
    let len: usize = 0; // Invalid length, should panic
    
    unsafe {
        let _iter_range: RawIterRange<u8> = RawIterRange::new(ctrl_ptr as *const u8, data_bucket, len);
    }
}

#[test]
#[should_panic]
fn test_raw_iter_range_new_unaligned_ctrl() {
    let layout = Layout::from_size_align(64, 8).unwrap();
    let ctrl_ptr = unsafe { std::alloc::alloc(layout).add(1) }; // Unaligned

    let data_bucket = Bucket {
        ptr: NonNull::new(ctrl_ptr).unwrap(),
    };

    let len: usize = 8; // must be a power of two

    unsafe {
        let _iter_range: RawIterRange<u8> = RawIterRange::new(ctrl_ptr as *const u8, data_bucket, len);
    }
}

#[test]
#[should_panic]
fn test_raw_iter_range_new_invalid_length() {
    let layout = Layout::from_size_align(64, 8).unwrap();
    let ctrl_ptr = unsafe { std::alloc::alloc(layout) };
    
    let data_bucket = Bucket {
        ptr: NonNull::new(ctrl_ptr).unwrap(),
    };

    let len: usize = 7; // Invalid length, not a power of two, should panic

    unsafe {
        let _iter_range: RawIterRange<u8> = RawIterRange::new(ctrl_ptr as *const u8, data_bucket, len);
    }

    unsafe {
        std::alloc::dealloc(ctrl_ptr, layout);
    }
}

