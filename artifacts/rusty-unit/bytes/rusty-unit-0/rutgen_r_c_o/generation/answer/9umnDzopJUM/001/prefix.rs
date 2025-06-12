// Answer 0

#[test]
fn test_shallow_clone_vec_success() {
    let atom = AtomicPtr::new(std::ptr::null_mut());
    let buf = Box::into_raw(Box::new([0u8; 10])); // valid non-null pointer
    let offset = buf as *const u8;
    let len = 5;

    let _ = unsafe { shallow_clone_vec(&atom, atom.load(Ordering::Relaxed), buf, offset, len) };
}

#[test]
fn test_shallow_clone_vec_failure_due_to_compare_exchange() {
    let atom = AtomicPtr::new(std::ptr::null_mut());
    let buf = Box::into_raw(Box::new([0u8; 10])); // valid non-null pointer
    let offset = buf as *const u8;
    let len = 5;

    // Simulate other thread having already promoted
    atom.store(buf as *mut _, Ordering::Release);

    let _ = unsafe { shallow_clone_vec(&atom, atom.load(Ordering::Relaxed), buf, offset, len) };
}

#[test]
#[should_panic]
fn test_shallow_clone_vec_invalid_len_zero() {
    let atom = AtomicPtr::new(std::ptr::null_mut());
    let buf = Box::into_raw(Box::new([0u8; 10])); // valid non-null pointer
    let offset = buf as *const u8;
    let len = 0; // Not valid

    let _ = unsafe { shallow_clone_vec(&atom, atom.load(Ordering::Relaxed), buf, offset, len) };
}

#[test]
fn test_shallow_clone_vec_len_max() {
    let atom = AtomicPtr::new(std::ptr::null_mut());
    let buf = Box::into_raw(Box::new([0u8; 10])); // valid non-null pointer
    let offset = buf as *const u8;
    let len = usize::MAX; // Edge case

    let _ = unsafe { shallow_clone_vec(&atom, atom.load(Ordering::Relaxed), buf, offset, len) };
}

#[test]
fn test_shallow_clone_vec_large_buffer() {
    let atom = AtomicPtr::new(std::ptr::null_mut());
    let buf = Box::into_raw(Box::new(vec![0u8; 1_000_000])); // valid non-null pointer
    let offset = buf as *const u8;
    let len = 100_000;

    let _ = unsafe { shallow_clone_vec(&atom, atom.load(Ordering::Relaxed), buf, offset, len) };
}

