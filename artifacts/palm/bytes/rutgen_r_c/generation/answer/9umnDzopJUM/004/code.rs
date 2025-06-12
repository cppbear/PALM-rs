// Answer 0

#[test]
fn test_shallow_clone_vec_success() {
    use core::ptr::null_mut;

    let atom = AtomicPtr::new(null_mut());
    let buffer: Vec<u8> = vec![1, 2, 3, 4, 5];
    let ptr: *const u8 = buffer.as_ptr();
    let buf: *mut u8 = buffer.as_mut_ptr();
    let offset: *const u8 = ptr.add(0); // Starting offset
    let len: usize = buffer.len();

    let result = unsafe { shallow_clone_vec(&atom, ptr as *const (), buf, offset, len) };
    
    assert_eq!(result.len, len);
    assert_eq!(result.ptr, offset);
    assert!(result.data.load(Ordering::Relaxed) != null_mut());
}

#[test]
#[should_panic]
fn test_shallow_clone_vec_overflow() {
    use core::ptr::null_mut;

    let atom = AtomicPtr::new(null_mut());
    let buffer: Vec<u8> = vec![0; usize::MAX]; // Large buffer
    let ptr: *const u8 = buffer.as_ptr();
    let buf: *mut u8 = buffer.as_mut_ptr();
    let offset: *const u8 = ptr.add(0); // Starting offset
    let len: usize = buffer.len();

    // This should trigger a panic due to the ref count exceeding the limit
    let _result = unsafe { shallow_clone_vec(&atom, ptr as *const (), buf, offset, len) };
}

#[test]
fn test_shallow_clone_vec_success_multiple_references() {
    use core::ptr::null_mut;

    let atom = AtomicPtr::new(null_mut());
    let buffer: Vec<u8> = vec![10, 20, 30, 40];
    let ptr: *const u8 = buffer.as_ptr();
    let buf: *mut u8 = buffer.as_mut_ptr();
    let offset: *const u8 = ptr.add(0); // Starting offset
    let len: usize = buffer.len();

    let first_clone = unsafe { shallow_clone_vec(&atom, ptr as *const (), buf, offset, len) };
    let second_clone = unsafe { shallow_clone_vec(&atom, ptr as *const (), buf, offset, len) };

    assert_eq!(first_clone.len, len);
    assert_eq!(first_clone.ptr, offset);
    assert_eq!(second_clone.len, len);
    assert_eq!(second_clone.ptr, offset);
}

