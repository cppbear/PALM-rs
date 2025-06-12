// Answer 0

#[test]
fn test_shared_to_vec_valid_conversion() {
    use std::ptr::null;
    use std::alloc::{alloc, dealloc, Layout};

    const LEN: usize = 10;
    let layout = Layout::from_size_align(LEN, 1).unwrap();
    let ptr = unsafe { alloc(layout) };
    
    let data = AtomicPtr::new(ptr as *mut ());
    let input_data = vec![1u8; LEN];
    unsafe {
        ptr.copy_from_nonoverlapping(input_data.as_ptr(), LEN);
    }

    let result = unsafe { shared_to_vec(&data, ptr, LEN) };
    
    assert_eq!(result.len(), LEN);
    assert_eq!(result, input_data);
    
    unsafe { dealloc(ptr, layout) };
}

#[test]
#[should_panic]
fn test_shared_to_vec_with_null_pointer() {
    use std::ptr::null;

    let data = AtomicPtr::new(null());
    let len = 10;

    unsafe {
        shared_to_vec(&data, null(), len);
    }
}

#[test]
fn test_shared_to_vec_empty_slice() {
    use std::alloc::{alloc, dealloc, Layout};

    const LEN: usize = 0;
    let layout = Layout::from_size_align(LEN, 1).unwrap();
    let ptr = unsafe { alloc(layout) };
    
    let data = AtomicPtr::new(ptr as *mut ());

    let result = unsafe { shared_to_vec(&data, ptr, LEN) };

    assert_eq!(result.len(), LEN);
    
    unsafe { dealloc(ptr, layout) };
}

#[test]
#[should_panic]
fn test_shared_to_vec_length_mismatch() {
    use std::alloc::{alloc, dealloc, Layout};

    const LEN: usize = 10;
    const MISMATCH: usize = 5;
    let layout = Layout::from_size_align(LEN, 1).unwrap();
    let ptr = unsafe { alloc(layout) };
    
    let data = AtomicPtr::new(ptr as *mut ());
    let input_data = vec![1u8; LEN];
    unsafe {
        ptr.copy_from_nonoverlapping(input_data.as_ptr(), LEN);
    }

    unsafe {
        // Intentionally providing a length mismatch
        shared_to_vec(&data, ptr, MISMATCH);
    }

    unsafe { dealloc(ptr, layout) };
}

