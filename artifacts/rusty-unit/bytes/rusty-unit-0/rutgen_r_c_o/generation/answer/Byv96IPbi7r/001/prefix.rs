// Answer 0

#[test]
fn test_owned_drop_impl_high_ref_count() {
    struct TestLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    unsafe fn test_drop(_: *mut ()) {}

    let mut test_instance = TestLifetime {
        ref_cnt: AtomicUsize::new(2),
        drop: test_drop,
    };
    let owned: *mut () = &mut test_instance as *mut _ as *mut ();

    owned_drop_impl(owned);
}

#[test]
fn test_owned_drop_impl_mid_ref_count() {
    struct TestLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    unsafe fn test_drop(_: *mut ()) {}

    let mut test_instance = TestLifetime {
        ref_cnt: AtomicUsize::new(4611686018427387903),
        drop: test_drop,
    };
    let owned: *mut () = &mut test_instance as *mut _ as *mut ();

    owned_drop_impl(owned);
}

#[test]
fn test_owned_drop_impl_edge_case() {
    struct TestLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    unsafe fn test_drop(_: *mut ()) {}

    let mut test_instance = TestLifetime {
        ref_cnt: AtomicUsize::new(4611686018427387901),
        drop: test_drop,
    };
    let owned: *mut () = &mut test_instance as *mut _ as *mut ();

    owned_drop_impl(owned);
}

