// Answer 0

#[test]
fn test_owned_drop_impl_valid_case() {
    use core::ptr::null_mut;
    use core::sync::atomic::Ordering;

    struct TestOwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    unsafe fn test_drop_fn(_: *mut ()) {
        // This is a placeholder for the drop function
    }

    let ref_cnt_value = (usize::MAX >> 1) + 1; // old_cnt > 0 and old_cnt != 1
    let owned_lifetime = TestOwnedLifetime {
        ref_cnt: AtomicUsize::new(ref_cnt_value),
        drop: test_drop_fn,
    };

    let owned_ptr = &owned_lifetime as *const _ as *mut ();

    unsafe {
        owned_drop_impl(owned_ptr);
        // Post condition: ref_cnt should be decremented but not to 1
        assert_eq!(owned_lifetime.ref_cnt.load(Ordering::Acquire), ref_cnt_value - 1);
    }
}

#[test]
#[should_panic(expected = "expected non-zero refcount and no underflow")]
fn test_owned_drop_impl_panic_underflow() {
    use core::ptr::null_mut;

    struct TestOwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    unsafe fn test_drop_fn(_: *mut ()) {
        // This is a placeholder for the drop function
    }

    let owned_lifetime = TestOwnedLifetime {
        ref_cnt: AtomicUsize::new(0), // Setting ref count to 0 to trigger panic
        drop: test_drop_fn,
    };

    let owned_ptr = &owned_lifetime as *const _ as *mut ();

    unsafe {
        owned_drop_impl(owned_ptr); // This should panic
    }
}

#[test]
fn test_owned_drop_impl_max_bound() {
    use core::ptr::null_mut;
    use core::sync::atomic::Ordering;

    struct TestOwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    unsafe fn test_drop_fn(_: *mut ()) {
        // This is a placeholder for the drop function
    }

    let owned_lifetime = TestOwnedLifetime {
        ref_cnt: AtomicUsize::new(usize::MAX >> 1), // old_cnt <= usize::MAX >> 1
        drop: test_drop_fn,
    };

    let owned_ptr = &owned_lifetime as *const _ as *mut ();

    unsafe {
        owned_drop_impl(owned_ptr);
        // Post condition: ref_cnt should be decremented by 1
        assert_eq!(owned_lifetime.ref_cnt.load(Ordering::Acquire), (usize::MAX >> 1) - 1);
    }
}

