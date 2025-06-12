// Answer 0

#[test]
fn test_promotable_odd_to_mut_valid_input() {
    let mut vec = Vec::with_capacity(10);
    vec.extend_from_slice(&[1u8, 2, 3, 4, 5]);
    let (ptr, len) = (vec.as_mut_ptr(), vec.len());
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());
    std::mem::forget(vec);

    unsafe {
        promotable_odd_to_mut(&atomic_ptr, ptr, len);
    }
}

#[test]
fn test_promotable_odd_to_mut_min_len() {
    let mut vec = Vec::with_capacity(1);
    vec.push(1u8);
    let (ptr, len) = (vec.as_mut_ptr(), vec.len());
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());
    std::mem::forget(vec);

    unsafe {
        promotable_odd_to_mut(&atomic_ptr, ptr, len);
    }
}

#[test]
fn test_promotable_odd_to_mut_large_len() {
    let mut vec = Vec::with_capacity(usize::MAX);
    let (ptr, len) = (vec.as_mut_ptr(), vec.capacity());
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());
    std::mem::forget(vec);

    unsafe {
        promotable_odd_to_mut(&atomic_ptr, ptr, len);
    }
}

#[test]
#[should_panic]
fn test_promotable_odd_to_mut_null_ptr() {
    let len = 10;
    let atomic_ptr = AtomicPtr::new(std::ptr::null_mut());

    unsafe {
        promotable_odd_to_mut(&atomic_ptr, std::ptr::null(), len);
    }
}

#[test]
#[should_panic]
fn test_promotable_odd_to_mut_invalid_len() {
    let mut vec = Vec::with_capacity(10);
    let (ptr, _) = (vec.as_mut_ptr(), vec.len());
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    unsafe {
        promotable_odd_to_mut(&atomic_ptr, ptr, usize::MAX);
    }
}

