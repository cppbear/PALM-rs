// Answer 0

#[derive(Debug)]
struct OwnedLifetime {
    ref_cnt: std::sync::atomic::AtomicUsize,
    drop: fn(*mut ()),
}

#[test]
fn test_owned_drop_impl_with_max_ref_cnt() {
    unsafe {
        let drop_fn = |ptr: *mut ()| {
            // Simulate the drop action.
            println!("Dropped memory at pointer: {:?}", ptr);
        };

        let lifetime = OwnedLifetime {
            ref_cnt: std::sync::atomic::AtomicUsize::new(usize::MAX >> 1),
            drop: drop_fn,
        };

        let owned: *mut () = &lifetime as *const _ as *mut _;

        // This invocation should not panic; conditions are satisfied:
        // old_cnt > 0, old_cnt <= usize::MAX >> 1, old_cnt != 1 is false
        owned_drop_impl(owned);
    }
}

#[test]
#[should_panic(expected = "expected non-zero refcount and no underflow")]
fn test_owned_drop_impl_with_zero_ref_cnt() {
    unsafe {
        let drop_fn = |ptr: *mut ()| {
            // Simulate the drop action.
            println!("Dropped memory at pointer: {:?}", ptr);
        };

        let lifetime = OwnedLifetime {
            ref_cnt: std::sync::atomic::AtomicUsize::new(0),
            drop: drop_fn,
        };

        let owned: *mut () = &lifetime as *const _ as *mut _;

        // This should panic as old_cnt > 0 is not satisfied.
        owned_drop_impl(owned);
    }
}

#[test]
#[should_panic(expected = "expected non-zero refcount and no underflow")]
fn test_owned_drop_impl_with_underflow_ref_cnt() {
    unsafe {
        let drop_fn = |ptr: *mut ()| {
            // Simulate the drop action.
            println!("Dropped memory at pointer: {:?}", ptr);
        };

        let lifetime = OwnedLifetime {
            ref_cnt: std::sync::atomic::AtomicUsize::new(1),
            drop: drop_fn,
        };

        let owned: *mut () = &lifetime as *const _ as *mut _;

        // This should panic because we will cause a ref_cnt underflow.
        owned_drop_impl(owned);
    }
}

