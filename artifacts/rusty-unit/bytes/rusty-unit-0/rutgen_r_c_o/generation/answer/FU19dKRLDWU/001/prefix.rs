// Answer 0

#[test]
fn test_is_unique_ref_count_one() {
    let vec = vec![1, 2, 3];
    let ref_count = AtomicUsize::new(1);
    let shared = Shared {
        vec,
        original_capacity_repr: 0,
        ref_count,
    };
    shared.is_unique();
}

#[test]
fn test_is_unique_ref_count_zero() {
    let vec = vec![1, 2, 3];
    let ref_count = AtomicUsize::new(0);
    let shared = Shared {
        vec,
        original_capacity_repr: 0,
        ref_count,
    };
    shared.is_unique();
}

#[test]
fn test_is_unique_ref_count_two() {
    let vec = vec![1, 2, 3];
    let ref_count = AtomicUsize::new(2);
    let shared = Shared {
        vec,
        original_capacity_repr: 0,
        ref_count,
    };
    shared.is_unique();
}

#[should_panic]
fn test_is_unique_ref_count_negative() {
    let vec = vec![1, 2, 3];
    let ref_count = AtomicUsize::new(usize::MAX);
    let shared = Shared {
        vec,
        original_capacity_repr: 0,
        ref_count,
    };
    shared.is_unique();
}

