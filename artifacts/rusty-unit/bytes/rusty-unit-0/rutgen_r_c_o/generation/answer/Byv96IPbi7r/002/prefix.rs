// Answer 0

#[test]
fn test_owned_drop_impl_with_valid_ref_count() {
    struct TestOwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    unsafe fn drop_fn(_: *mut ()) {}

    let owned_lifetime = Box::into_raw(Box::new(TestOwnedLifetime {
        ref_cnt: AtomicUsize::new(usize::MAX >> 1),
        drop: drop_fn,
    }));

    owned_drop_impl(owned_lifetime as *mut ());
}

#[test]
fn test_owned_drop_impl_with_edge_case_ref_count() {
    struct TestOwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    unsafe fn drop_fn(_: *mut ()) {}

    let owned_lifetime = Box::into_raw(Box::new(TestOwnedLifetime {
        ref_cnt: AtomicUsize::new(2),
        drop: drop_fn,
    }));

    owned_drop_impl(owned_lifetime as *mut ());
}

#[test]
#[should_panic]
fn test_owned_drop_impl_with_zero_ref_count() {
    struct TestOwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    unsafe fn drop_fn(_: *mut ()) {}

    let owned_lifetime = Box::into_raw(Box::new(TestOwnedLifetime {
        ref_cnt: AtomicUsize::new(0),
        drop: drop_fn,
    }));

    owned_drop_impl(owned_lifetime as *mut ());
}

#[test]
fn test_owned_drop_impl_with_one_ref_count() {
    struct TestOwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    unsafe fn drop_fn(_: *mut ()) {}

    let owned_lifetime = Box::into_raw(Box::new(TestOwnedLifetime {
        ref_cnt: AtomicUsize::new(1),
        drop: drop_fn,
    }));

    owned_drop_impl(owned_lifetime as *mut ());
}

