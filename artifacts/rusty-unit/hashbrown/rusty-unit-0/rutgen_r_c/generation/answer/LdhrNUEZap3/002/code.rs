// Answer 0

#[test]
fn test_fold_impl_empty() {
    struct TestBucket;

    let data = Bucket {
        ptr: NonNull::dangling(),
    };

    let raw_iter_range = RawIterRange {
        current_group: BitMaskIter(BitMask(0)), // No bits set
        data,
        next_ctrl: ptr::null(),
        end: ptr::null(),
    };

    let result: i32 = unsafe { raw_iter_range.fold_impl(0, 0, |acc, _| acc + 1) };
    assert_eq!(result, 0);
}

#[test]
fn test_fold_impl_single() {
    struct TestBucket;

    let data = Bucket {
        ptr: NonNull::new_unchecked(&TestBucket as *const _ as *mut _),
    };

    let raw_iter_range = RawIterRange {
        current_group: BitMaskIter(BitMask(1)), // One bit set
        data,
        next_ctrl: ptr::null(),
        end: ptr::null(),
    };

    let result: i32 = unsafe { raw_iter_range.fold_impl(1, 0, |acc, _| acc + 1) };
    assert_eq!(result, 1);
}

#[test]
fn test_fold_impl_multiple() {
    struct TestBucket;

    let data = Bucket {
        ptr: NonNull::new_unchecked(&TestBucket as *const _ as *mut _),
    };

    let raw_iter_range = RawIterRange {
        current_group: BitMaskIter(BitMask(0b111)), // Three bits set
        data,
        next_ctrl: ptr::null(),
        end: ptr::null(),
    };

    let result: i32 = unsafe { raw_iter_range.fold_impl(3, 0, |acc, _| acc + 1) };
    assert_eq!(result, 3);
}

#[test]
#[should_panic]
fn test_fold_impl_panic_empty_iter() {
    struct TestBucket;

    let data = Bucket {
        ptr: NonNull::dangling(),
    };

    let raw_iter_range = RawIterRange {
        current_group: BitMaskIter(BitMask(0)),
        data,
        next_ctrl: ptr::null(),
        end: ptr::null(),
    };

    unsafe {
        raw_iter_range.fold_impl(1, 0, |acc, _| acc + 1);
    }
}

#[test]
#[should_panic]
fn test_fold_impl_panic_extra_count() {
    struct TestBucket;

    let data = Bucket {
        ptr: NonNull::new_unchecked(&TestBucket as *const _ as *mut _),
    };

    let raw_iter_range = RawIterRange {
        current_group: BitMaskIter(BitMask(1)),
        data,
        next_ctrl: ptr::null(),
        end: ptr::null(),
    };

    unsafe {
        raw_iter_range.fold_impl(2, 0, |acc, _| acc + 1);
    }
}

