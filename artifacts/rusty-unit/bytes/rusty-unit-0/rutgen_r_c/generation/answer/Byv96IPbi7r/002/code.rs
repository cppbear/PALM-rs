// Answer 0

#[test]
fn test_owned_drop_impl() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestLifetime {
        ref_cnt: AtomicUsize,
        drop_fn: unsafe fn(*mut ()),
    }

    unsafe fn test_drop_fn(_: *mut ()) {
        // dummy drop function for testing
    }

    let drop_fn: unsafe fn(*mut ()) = test_drop_fn;

    // Creating an instance of OwnedLifetime with old_cnt set to usize::MAX >> 1
    let instance = TestLifetime {
        ref_cnt: AtomicUsize::new(usize::MAX >> 1),
        drop_fn,
    };

    // Unsafe block to create a raw pointer to simulate the condition
    let owned_ptr: *mut () = &instance as *const _ as *mut _;

    // Invoke the function, ensuring it does not panic
    unsafe { owned_drop_impl(owned_ptr) };

    // After function call, verify the ref count has been decremented to 0
    assert_eq!(instance.ref_cnt.load(Ordering::Relaxed), usize::MAX >> 1 - 1);
}

#[should_panic(expected = "expected non-zero refcount and no underflow")]
#[test]
fn test_owned_drop_impl_underflow() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestLifetime {
        ref_cnt: AtomicUsize,
        drop_fn: unsafe fn(*mut ()),
    }

    unsafe fn test_drop_fn(_: *mut ()) {
        // dummy drop function for testing
    }

    let drop_fn: unsafe fn(*mut ()) = test_drop_fn;

    // Creating an instance of OwnedLifetime with old_cnt set to 0 to trigger panic
    let instance = TestLifetime {
        ref_cnt: AtomicUsize::new(0),
        drop_fn,
    };

    // Unsafe block to create a raw pointer to simulate the condition
    let owned_ptr: *mut () = &instance as *const _ as *mut _;

    // This call should panic due to the first debug_assert in owned_drop_impl
    unsafe { owned_drop_impl(owned_ptr) };
}

