// Answer 0

#[repr(C)]
struct OwnedLifetime {
    ref_cnt: std::sync::atomic::AtomicUsize,
    drop: fn(*mut ()),
}

#[test]
#[should_panic(expected = "expected non-zero refcount and no underflow")]
fn test_owned_drop_impl_panic_due_to_zero_ref_count() {
    unsafe {
        let drop_fn: fn(*mut ()) = |ptr| {
            // Dummy drop function to simulate behavior
            drop(Box::from_raw(ptr as *mut u8));
        };
        
        let owned_lifetime = OwnedLifetime {
            ref_cnt: std::sync::atomic::AtomicUsize::new(1),
            drop: drop_fn,
        };

        let owned_ptr = &owned_lifetime as *const _ as *mut ();

        // Decrement ref count to 0, forcing the panic condition
        owned_lifetime.ref_cnt.fetch_sub(1, std::sync::atomic::Ordering::Release);
        
        owned_drop_impl(owned_ptr);
    }
}

#[test]
#[should_panic(expected = "expected non-zero refcount and no underflow")]
fn test_owned_drop_impl_panic_due_to_underflow() {
    unsafe {
        let drop_fn: fn(*mut ()) = |ptr| {
            // Dummy drop function to simulate behavior
            drop(Box::from_raw(ptr as *mut u8));
        };

        let owned_lifetime = OwnedLifetime {
            ref_cnt: std::sync::atomic::AtomicUsize::new(usize::MAX >> 1),
            drop: drop_fn,
        };

        let owned_ptr = &owned_lifetime as *const _ as *mut ();

        // Decrement ref count to exceed the upper limit, which is a boundary condition
        owned_lifetime.ref_cnt.fetch_sub(usize::MAX >> 1, std::sync::atomic::Ordering::Release);
        
        owned_drop_impl(owned_ptr);
    }
}

