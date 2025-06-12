// Answer 0

#[test]
fn test_owned_drop() {
    use core::ptr::null_mut;
    use core::sync::atomic::{AtomicPtr, Ordering};

    struct OwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: fn(*mut ()),
    }

    unsafe extern "C" fn drop_fn(ptr: *mut ()) {
        // Mock drop function that does nothing
    }

    // Initialize an AtomicPtr with a valid OwnedLifetime
    let owned_lifetime = Box::into_raw(Box::new(OwnedLifetime {
        ref_cnt: AtomicUsize::new(1),
        drop: drop_fn,
    }));

    let mut data = AtomicPtr::new(owned_lifetime as *mut ());

    // Call the owned_drop function
    unsafe {
        owned_drop(&mut data, null_mut(), 0);
    }

    // Ensure that the reference count has decremented properly
    assert_eq!(owned_lifetime.as_ref().unwrap().ref_cnt.load(Ordering::Relaxed), 0);
    unsafe {
        drop(Box::from_raw(owned_lifetime)); // Clean up the mock allocation
    }
}

#[test]
#[should_panic(expected = "expected non-zero refcount and no underflow")]
fn test_owned_drop_underflow() {
    use core::ptr::null_mut;
    use core::sync::atomic::{AtomicPtr, Ordering};

    struct OwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: fn(*mut ()),
    }

    unsafe extern "C" fn drop_fn(ptr: *mut ()) {
        // Mock drop function that does nothing
    }

    // Initialize an AtomicPtr with a valid OwnedLifetime 
    let owned_lifetime = Box::into_raw(Box::new(OwnedLifetime {
        ref_cnt: AtomicUsize::new(0), // Setting it to 0 to induce underflow
        drop: drop_fn,
    }));

    let mut data = AtomicPtr::new(owned_lifetime as *mut ());

    // Call the owned_drop function which should panic
    unsafe {
        owned_drop(&mut data, null_mut(), 0);
    }
}

