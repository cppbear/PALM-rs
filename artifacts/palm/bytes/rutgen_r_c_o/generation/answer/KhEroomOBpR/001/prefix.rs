// Answer 0

#[test]
fn test_increment_shared_maximum_old_size() {
    let shared = Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(isize::MAX as usize + 1),
    };
    let ptr = &shared as *const Shared as *mut Shared;
    unsafe { increment_shared(ptr) };
}

#[test]
#[should_panic]
fn test_increment_shared_overflow() {
    let shared = Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(usize::MAX),
    };
    let ptr = &shared as *const Shared as *mut Shared;
    unsafe { increment_shared(ptr) };
}

