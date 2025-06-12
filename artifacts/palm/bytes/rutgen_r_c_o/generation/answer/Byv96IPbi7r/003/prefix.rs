// Answer 0

#[test]
fn test_owned_drop_impl_valid_case() {
    struct TestLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    unsafe extern "C" fn dummy_drop(_: *mut ()) {}

    let test_data = TestLifetime {
        ref_cnt: AtomicUsize::new(2), // old_cnt will be 2 for this case
        drop: dummy_drop,
    };

    let owned: *mut () = &test_data as *const _ as *mut ();
    unsafe {
        owned_drop_impl(owned);
    }
}

#[test]
#[should_panic(expected = "expected non-zero refcount and no underflow")]
fn test_owned_drop_impl_ref_count_zero() {
    struct TestLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    unsafe extern "C" fn dummy_drop(_: *mut ()) {}

    let test_data = TestLifetime {
        ref_cnt: AtomicUsize::new(0), // This will trigger a panic
        drop: dummy_drop,
    };

    let owned: *mut () = &test_data as *const _ as *mut ();
    unsafe {
        owned_drop_impl(owned);
    }
}

#[test]
fn test_owned_drop_impl_boundary_case() {
    struct TestLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    unsafe extern "C" fn dummy_drop(_: *mut ()) {}

    let test_data = TestLifetime {
        ref_cnt: AtomicUsize::new(usize::MAX >> 1), // old_cnt will be the boundary value
        drop: dummy_drop,
    };

    let owned: *mut () = &test_data as *const _ as *mut ();
    unsafe {
        owned_drop_impl(owned);
    }
}

#[test]
fn test_owned_drop_impl_exceeding_limit() {
    struct TestLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    unsafe extern "C" fn dummy_drop(_: *mut ()) {}

    let test_data = TestLifetime {
        ref_cnt: AtomicUsize::new(usize::MAX >> 1 + 1), // This will satisfy old_cnt <= usize::MAX >> 1 is false
        drop: dummy_drop,
    };

    let owned: *mut () = &test_data as *const _ as *mut ();
    unsafe {
        owned_drop_impl(owned);
    }
}

