// Answer 0

#[test]
fn test_fold_impl_with_valid_acc_and_n() {
    struct TestBucket;
    
    let data = Bucket {
        ptr: NonNull::new_unchecked(&mut TestBucket as *mut _),
    };
    let current_group = BitMaskIter(BitMask(0b1111)); // 4 items available
    let end = ptr::null();
    let next_ctrl = ptr::null();
    
    let mut raw_iter = RawIterRange {
        current_group,
        data,
        next_ctrl,
        end,
    };
    
    let n = 4; 
    let mut acc = 0; 
    let result = unsafe {
        raw_iter.fold_impl(n, acc, |acc, _: Bucket<TestBucket>| {
            acc + 1
        })
    };
}

#[test]
fn test_fold_impl_with_one_item() {
    struct TestBucket;

    let data = Bucket {
        ptr: NonNull::new_unchecked(&mut TestBucket as *mut _),
    };
    let current_group = BitMaskIter(BitMask(0b0001)); // 1 item available
    let end = ptr::null();
    let next_ctrl = ptr::null();

    let mut raw_iter = RawIterRange {
        current_group,
        data,
        next_ctrl,
        end,
    };

    let n = 1; 
    let mut acc = 5; 
    let result = unsafe {
        raw_iter.fold_impl(n, acc, |acc, _: Bucket<TestBucket>| {
            acc + 1
        })
    };
}

#[test]
fn test_fold_impl_with_multiple_items() {
    struct TestBucket;

    let data = Bucket {
        ptr: NonNull::new_unchecked(&mut TestBucket as *mut _),
    };
    let current_group = BitMaskIter(BitMask(0b1111)); // 4 items available
    let end = ptr::null();
    let next_ctrl = ptr::null();

    let mut raw_iter = RawIterRange {
        current_group,
        data,
        next_ctrl,
        end,
    };

    let n = 3; 
    let mut acc = 10; 
    let result = unsafe {
        raw_iter.fold_impl(n, acc, |acc, _: Bucket<TestBucket>| {
            acc + 1
        })
    };
}

#[test]
fn test_fold_impl_with_edge_case_of_n() {
    struct TestBucket;

    let data = Bucket {
        ptr: NonNull::new_unchecked(&mut TestBucket as *mut _),
    };
    let current_group = BitMaskIter(BitMask(0b1111)); // 4 items available
    let end = ptr::null();
    let next_ctrl = ptr::null();

    let mut raw_iter = RawIterRange {
        current_group,
        data,
        next_ctrl,
        end,
    };

    let n = 1; 
    let mut acc = 20; 
    let result = unsafe {
        raw_iter.fold_impl(n, acc, |acc, _: Bucket<TestBucket>| {
            acc + 2
        })
    };
}

#[test]
fn test_fold_impl_with_no_items() {
    struct TestBucket;

    let data = Bucket {
        ptr: NonNull::new_unchecked(&mut TestBucket as *mut _),
    };
    let current_group = BitMaskIter(BitMask(0b0000)); // No items available
    let end = ptr::null();
    let next_ctrl = ptr::null();

    let mut raw_iter = RawIterRange {
        current_group,
        data,
        next_ctrl,
        end,
    };

    let n = 0; 
    let mut acc = 15; 
    let result = unsafe {
        raw_iter.fold_impl(n, acc, |acc, _: Bucket<TestBucket>| {
            acc + 1
        })
    };
}

