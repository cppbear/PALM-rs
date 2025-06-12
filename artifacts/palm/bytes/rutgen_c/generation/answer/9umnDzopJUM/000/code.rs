// Answer 0

#[test]
fn test_shallow_clone_vec_success() {
    let atom = AtomicPtr::new(ptr::null_mut());
    let buf: *mut u8 = unsafe { alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(10, 1).unwrap()) };
    let offset: *const u8 = unsafe { buf.add(0) };
    let len = 10;

    let cloned_bytes = unsafe { shallow_clone_vec(&atom, buf as *const (), buf, offset, len) };

    assert_eq!(cloned_bytes.len, len);
    assert_eq!(cloned_bytes.ptr, offset);
    assert!(!cloned_bytes.data.load(Ordering::Relaxed).is_null());

    unsafe { alloc::alloc::dealloc(buf, alloc::alloc::Layout::from_size_align(10, 1).unwrap()) };
}

#[test]
fn test_shallow_clone_vec_concurrent_clone() {
    let atom = AtomicPtr::new(ptr::null_mut());
    let buf: *mut u8 = unsafe { alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(10, 1).unwrap()) };
    let offset: *const u8 = unsafe { buf.add(0) };
    let len = 10;

    // Simulate a successful promotion of buffer to `Arc` by setting
    // the atom pointer directly.
    atom.store(buf as *mut _, Ordering::Release);

    let cloned_bytes = unsafe { shallow_clone_vec(&atom, buf as *const (), buf, offset, len) };

    assert_eq!(cloned_bytes.len, len);
    assert_eq!(cloned_bytes.ptr, offset);
    assert!(!cloned_bytes.data.load(Ordering::Relaxed).is_null());

    unsafe { alloc::alloc::dealloc(buf, alloc::alloc::Layout::from_size_align(10, 1).unwrap()) };
}

#[should_panic]
#[test]
fn test_shallow_clone_vec_invalid_buffer() {
    let atom = AtomicPtr::new(ptr::null_mut());
    let buf: *mut u8 = ptr::null_mut(); // Invalid buffer
    let offset: *const u8 = ptr::null();
    let len = 0;

    unsafe { shallow_clone_vec(&atom, buf as *const (), buf, offset, len) };
}

#[test]
fn test_shallow_clone_vec_empty() {
    let atom = AtomicPtr::new(ptr::null_mut());
    let buf: *mut u8 = unsafe { alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(0, 1).unwrap()) };
    let offset: *const u8 = ptr::null();
    let len = 0;

    let cloned_bytes = unsafe { shallow_clone_vec(&atom, buf as *const (), buf, offset, len) };

    assert_eq!(cloned_bytes.len, len);
    assert_eq!(cloned_bytes.ptr, offset);
    assert!(!cloned_bytes.data.load(Ordering::Relaxed).is_null());

    unsafe { alloc::alloc::dealloc(buf, alloc::alloc::Layout::from_size_align(0, 1).unwrap()) };
}

