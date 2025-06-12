// Answer 0

#[test]
fn test_release_shared_with_ref_count_2() {
    let shared = Box::new(Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(2),
    });
    let ptr = Box::into_raw(shared);
    unsafe {
        release_shared(ptr);
    }
}

#[test]
fn test_release_shared_with_ref_count_max() {
    let shared = Box::new(Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(usize::MAX),
    });
    let ptr = Box::into_raw(shared);
    unsafe {
        release_shared(ptr);
    }
}

#[test]
fn test_release_shared_with_ref_count_3() {
    let shared = Box::new(Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(3),
    });
    let ptr = Box::into_raw(shared);
    unsafe {
        release_shared(ptr);
    }
}

#[test]
fn test_release_shared_with_ref_count_4() {
    let shared = Box::new(Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(4),
    });
    let ptr = Box::into_raw(shared);
    unsafe {
        release_shared(ptr);
    }
}

#[test]
fn test_release_shared_with_ref_count_5() {
    let shared = Box::new(Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(5),
    });
    let ptr = Box::into_raw(shared);
    unsafe {
        release_shared(ptr);
    }
}

