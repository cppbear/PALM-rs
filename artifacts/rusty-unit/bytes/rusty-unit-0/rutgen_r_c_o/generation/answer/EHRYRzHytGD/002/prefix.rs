// Answer 0

#[test]
fn test_release_shared_ref_count_is_one() {
    use core::sync::atomic::AtomicUsize;

    let shared = Box::new(Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });

    let ptr = Box::into_raw(shared);
    unsafe {
        release_shared(ptr);
    }
}

#[test]
fn test_release_shared_multiple_times() {
    use core::sync::atomic::AtomicUsize;

    let shared = Box::new(Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(2),
    });

    let ptr = Box::into_raw(shared);
    
    unsafe {
        // First release should not drop the box
        release_shared(ptr);
        // Second release should drop the box
        release_shared(ptr);
    }
}

#[test]
fn test_release_shared_with_empty_vec() {
    use core::sync::atomic::AtomicUsize;

    let shared = Box::new(Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });

    let ptr = Box::into_raw(shared);
    unsafe {
        release_shared(ptr);
    }
}

#[test]
fn test_release_shared_with_large_vec() {
    use core::sync::atomic::AtomicUsize;

    let large_vec = vec![0u8; 1024]; // Large Vec
    let shared = Box::new(Shared {
        vec: large_vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });

    let ptr = Box::into_raw(shared);
    unsafe {
        release_shared(ptr);
    }
}

#[test]
fn test_release_shared_with_high_ref_count() {
    use core::sync::atomic::AtomicUsize;

    let shared = Box::new(Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(5),
    });

    let ptr = Box::into_raw(shared);
    
    unsafe {
        // Only the first call should not drop the box
        release_shared(ptr);
        release_shared(ptr);
        release_shared(ptr);
        release_shared(ptr);
        // Last call should drop the box
        release_shared(ptr);
    }
}

