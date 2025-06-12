// Answer 0

#[test]
fn test_owned_drop_impl() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct OwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: fn(*mut ()),
    }

    extern "C" fn mock_drop(_ptr: *mut ()) {
        // Mock drop function for testing
    }

    let mut lifetime = OwnedLifetime {
        ref_cnt: AtomicUsize::new(1),
        drop: mock_drop,
    };

    let owned: *mut () = &mut lifetime as *mut _ as *mut ();

    // Test normal drop case
    unsafe {
        owned_drop_impl(owned);
    }
    assert_eq!(lifetime.ref_cnt.load(Ordering::Relaxed), 0);
}

#[test]
#[should_panic]
fn test_owned_drop_impl_underflow() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct OwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: fn(*mut ()),
    }

    extern "C" fn mock_drop(_ptr: *mut ()) {
        // Mock drop function for testing
    }

    let mut lifetime = OwnedLifetime {
        ref_cnt: AtomicUsize::new(0), // Start with 0 to cause underflow
        drop: mock_drop,
    };

    let owned: *mut () = &mut lifetime as *mut _ as *mut ();

    // This call should panic due to underflow check
    unsafe {
        owned_drop_impl(owned);
    }
}

