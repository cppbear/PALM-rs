// Answer 0

#[test]
fn test_from_base_index_zero_sized() {
    struct ZST;

    impl ZST {
        const IS_ZERO_SIZED: bool = true;
    }

    let base = NonNull::new_unchecked(&mut ZST as *mut ZST);
    let index = 0;

    let bucket: Bucket<ZST> = unsafe { Bucket::from_base_index(base, index) };

    assert_eq!(bucket.ptr.as_ptr() as usize, (index + 1) as *mut ZST as usize);
}

#[test]
fn test_from_base_index_zero_sized_boundary() {
    struct ZST;

    impl ZST {
        const IS_ZERO_SIZED: bool = true;
    }

    let base = NonNull::new_unchecked(&mut ZST as *mut ZST);
    let index = 1;

    let bucket: Bucket<ZST> = unsafe { Bucket::from_base_index(base, index) };

    assert_eq!(bucket.ptr.as_ptr() as usize, (index + 1) as *mut ZST as usize);
}

#[should_panic]
#[test]
fn test_from_base_index_zero_sized_overflow() {
    struct ZST;

    impl ZST {
        const IS_ZERO_SIZED: bool = true;
    }

    let base = NonNull::new_unchecked(&mut ZST as *mut ZST);
    let index = 2; // Assuming index > bucket_mask should panic

    unsafe { Bucket::from_base_index(base, index) }; // This should panic
}

