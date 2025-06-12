// Answer 0

#[test]
fn test_owned_clone_panic() {
    use core::ptr::null_mut;

    struct TestOwnedLifetime {
        ref_cnt: AtomicUsize,
    }

    let owned_lifetime = Box::new(TestOwnedLifetime {
        ref_cnt: AtomicUsize::new(usize::MAX >> 1),
    });

    let atomic_ptr = AtomicPtr::new(Box::into_raw(owned_lifetime) as _);
    let data = &atomic_ptr;
    let ptr: *const u8 = null_mut();
    let len = 0;

    unsafe {
        // This should trigger the abort condition as old_cnt will exceed usize::MAX >> 1
        let result = owned_clone(data, ptr, len);
        assert_eq!(result.len, len);
        assert_eq!(result.ptr, ptr);
    }
}

