// Answer 0

#[test]
fn test_raw_iter_range_new_valid() {
    use std::alloc::{alloc, dealloc, Layout};
    
    const WIDTH: usize = 8; // Assuming Group::WIDTH is 8
    let layout = Layout::from_size_align(64, WIDTH).unwrap(); // Allocate space for control bytes
    let ctrl = unsafe { alloc(layout) as *const u8 };
    let data = Bucket::<u32>::default(); // Assuming Bucket has a default implementation for some type T

    unsafe {
        // Initialize control bytes to ensure they are properly aligned
        for i in 0..WIDTH {
            *ctrl.offset(i as isize) = i as u8; 
        }
        
        let len = 4; // Power of two
        let raw_iter_range = RawIterRange::new(ctrl, data, len);
        
        assert!(!raw_iter_range.next_ctrl.is_null());
        assert_eq!(raw_iter_range.end, ctrl.add(len));
    }

    unsafe { dealloc(ctrl as *mut u8, layout) }; // Clean up the allocation
}

#[should_panic]
#[test]
fn test_raw_iter_range_new_invalid_length_zero() {
    use std::alloc::{alloc, dealloc, Layout};

    const WIDTH: usize = 8;
    let layout = Layout::from_size_align(64, WIDTH).unwrap();
    let ctrl = unsafe { alloc(layout) as *const u8 };
    let data = Bucket::<u32>::default();

    unsafe {
        // Attempt to create RawIterRange with length zero
        let _ = RawIterRange::new(ctrl, data, 0);
    }

    unsafe { dealloc(ctrl as *mut u8, layout) };
}

#[should_panic]
#[test]
fn test_raw_iter_range_new_invalid_non_power_of_two_length() {
    use std::alloc::{alloc, dealloc, Layout};

    const WIDTH: usize = 8;
    let layout = Layout::from_size_align(64, WIDTH).unwrap();
    let ctrl = unsafe { alloc(layout) as *const u8 };
    let data = Bucket::<u32>::default();

    unsafe {
        // Attempt to create RawIterRange with a length that is not a power of two
        let _ = RawIterRange::new(ctrl, data, 3);
    }

    unsafe { dealloc(ctrl as *mut u8, layout) };
}

#[should_panic]
#[test]
fn test_raw_iter_range_new_invalid_alignment() {
    use std::alloc::{alloc, dealloc, Layout};

    const WIDTH: usize = 8;
    let layout = Layout::from_size_align(64, WIDTH).unwrap();
    let ctrl = unsafe { alloc(layout.wrapping_add(1)) as *const u8 }; // Misalignment
    let data = Bucket::<u32>::default();

    unsafe {
        // Attempt to create RawIterRange with misaligned control pointer
        let _ = RawIterRange::new(ctrl, data, 4);
    }

    unsafe { dealloc(ctrl as *mut u8, layout) };
}

