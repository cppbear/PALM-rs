// Answer 0

#[test]
fn test_slice_assume_init_valid() {
    use std::mem::MaybeUninit;

    let initialized: [MaybeUninit<i32>; 3] = [
        MaybeUninit::new(1),
        MaybeUninit::new(2),
        MaybeUninit::new(3),
    ];

    let result: &[i32] = unsafe { slice_assume_init(&initialized) };

    assert_eq!(result, &[1, 2, 3]);
}

#[test]
#[should_panic]
fn test_slice_assume_init_uninitialized() {
    use std::mem::MaybeUninit;

    let uninitialized: [MaybeUninit<i32>; 3] = [
        MaybeUninit::uninit(),
        MaybeUninit::new(2),
        MaybeUninit::new(3),
    ];

    let _result: &[i32] = unsafe { slice_assume_init(&uninitialized) };
}

#[test]
fn test_slice_assume_init_edge_case() {
    use std::mem::MaybeUninit;

    let empty: [MaybeUninit<i32>; 0] = [];

    let result: &[i32] = unsafe { slice_assume_init(&empty) };

    assert_eq!(result, &[]);
}

#[test]
#[should_panic]
fn test_slice_assume_init_mixed_initialization() {
    use std::mem::MaybeUninit;

    let mixed: [MaybeUninit<i32>; 3] = [
        MaybeUninit::new(1),
        MaybeUninit::uninit(),
        MaybeUninit::new(3),
    ];

    let _result: &[i32] = unsafe { slice_assume_init(&mixed) };
}

