// Answer 0

#[test]
fn test_raw_iter_range_new_valid_input() {
    use std::alloc::{alloc, dealloc, Layout};

    let layout = Layout::from_size_align(64, 8).unwrap(); // Allocate 64 bytes aligned for control bytes
    let ctrl_ptr = unsafe { alloc(layout) };

    let bucket = Bucket {
        ptr: NonNull::new(ctrl_ptr as *mut u8).unwrap(),
    };

    let len = 4; // Power of two, valid length
    unsafe {
        let iter_range = RawIterRange::new(ctrl_ptr, bucket, len);
    }

    unsafe {
        dealloc(ctrl_ptr, layout);
    }
}

#[test]
#[should_panic]
fn test_raw_iter_range_new_zero_length() {
    let layout = Layout::from_size_align(64, 8).unwrap();
    let ctrl_ptr = unsafe { alloc(layout) };

    let bucket = Bucket {
        ptr: NonNull::new(ctrl_ptr as *mut u8).unwrap(),
    };

    let len = 0; // Invalid length
    unsafe {
        RawIterRange::new(ctrl_ptr, bucket, len);
    }

    unsafe {
        dealloc(ctrl_ptr, layout);
    }
}

#[test]
#[should_panic]
fn test_raw_iter_range_new_unaligned_ctrl() {
    let layout = Layout::from_size_align(64, 8).unwrap();
    let ctrl_ptr = unsafe { alloc(layout) };

    let bucket = Bucket {
        ptr: NonNull::new(ctrl_ptr as *mut u8).unwrap(),
    };

    let len = 4; // Power of two, valid length
    let unaligned_ctrl_ptr = ctrl_ptr.add(1); // Alter to create unaligned pointer

    unsafe {
        RawIterRange::new(unaligned_ctrl_ptr, bucket, len);
    }

    unsafe {
        dealloc(ctrl_ptr, layout);
    }
}

#[test]
#[should_panic]
fn test_raw_iter_range_new_exceeding_length() {
    let layout = Layout::from_size_align(64, 8).unwrap();
    let ctrl_ptr = unsafe { alloc(layout) };

    let bucket = Bucket {
        ptr: NonNull::new(ctrl_ptr as *mut u8).unwrap(),
    };

    let len = 64; // Exceeds the number of table buckets
    unsafe {
        RawIterRange::new(ctrl_ptr, bucket, len);
    }

    unsafe {
        dealloc(ctrl_ptr, layout);
    }
}

