// Answer 0

#[test]
fn test_next_n_zero_sized() {
    use core::ptr::NonNull;

    struct ZeroSized;

    impl ZeroSized {
        const IS_ZERO_SIZED: bool = true;
    }

    unsafe fn create_bucket() -> Bucket<ZeroSized> {
        let base = NonNull::new_unchecked(1 as *mut ZeroSized);
        Bucket::from_base_index(base, 0)
    }

    let bucket = unsafe { create_bucket() };
    let offset = 2; // Example offset

    let new_bucket = unsafe { bucket.next_n(offset) };

    assert!(!new_bucket.ptr.as_ptr().is_null());
}

#[test]
fn test_next_n_zero_sized_large_offset() {
    use core::ptr::NonNull;

    struct ZeroSized;

    impl ZeroSized {
        const IS_ZERO_SIZED: bool = true;
    }

    unsafe fn create_bucket() -> Bucket<ZeroSized> {
        let base = NonNull::new_unchecked(1 as *mut ZeroSized);
        Bucket::from_base_index(base, 0)
    }

    let bucket = unsafe { create_bucket() };
    let offset = 1000; // Large offset

    let new_bucket = unsafe { bucket.next_n(offset) };

    assert!(!new_bucket.ptr.as_ptr().is_null());
}

