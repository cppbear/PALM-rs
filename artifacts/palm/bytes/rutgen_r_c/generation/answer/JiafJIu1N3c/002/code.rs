// Answer 0

#[test]
fn test_promotable_to_mut_kind_vec() {
    use core::ptr::null_mut;
    use core::mem::MaybeUninit;
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct MockShared {
        ref_cnt: AtomicUsize,
        buf: *mut u8,
        cap: usize,
    }

    let buffer: Vec<u8> = vec![1, 2, 3, 4, 5];
    let buf_ptr = buffer.as_ptr() as *mut u8;
    let length = buffer.len();
    
    let shared = Box::into_raw(Box::new(MockShared {
        ref_cnt: AtomicUsize::new(1),
        buf: buf_ptr,
        cap: length,
    }));

    let data = AtomicPtr::new(shared as *mut ());

    unsafe {
        let output = promotable_to_mut(&data, buf_ptr, length, |ptr| {
            ptr as *mut u8
        });

        assert_eq!(output.len(), length);
        assert!(!output.is_empty());
        for i in 0..length {
            assert_eq!(output.as_slice()[i], buffer[i]);
        }
    }

    // Cleanup
    unsafe {
        let _ = Box::from_raw(shared);
    }
}

#[test]
#[should_panic]
fn test_promotable_to_mut_advance_unchecked_panic() {
    use core::ptr::null_mut;
    use core::mem::MaybeUninit;
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct MockShared {
        ref_cnt: AtomicUsize,
        buf: *mut u8,
        cap: usize,
    }

    let buffer: Vec<u8> = vec![1, 2, 3, 4, 5];
    let buf_ptr = buffer.as_ptr() as *mut u8;
    let length = buffer.len();

    // Creating a shared mock where we set an invalid condition for offset
    let shared = Box::into_raw(Box::new(MockShared {
        ref_cnt: AtomicUsize::new(1),
        buf: buf_ptr,
        cap: length,
    }));

    let data = AtomicPtr::new(shared as *mut ());

    unsafe {
        // Trying to promote with an invalid offset (greater than capacity)
        let _ = promotable_to_mut(&data, buf_ptr.add(10), length, |ptr| {
            ptr as *mut u8
        });
    }

    // Cleanup
    unsafe {
        let _ = Box::from_raw(shared);
    }
}

