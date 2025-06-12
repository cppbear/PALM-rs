// Answer 0

#[test]
#[should_panic(expected = "expected non-zero refcount and no underflow")]
fn test_owned_drop_impl_zero_ref_count() {
    use core::sync::atomic::AtomicUsize;

    struct OwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    unsafe extern "C" fn drop_fn(_: *mut ()) {}

    let owned_lifetime = OwnedLifetime {
        ref_cnt: AtomicUsize::new(0),
        drop: drop_fn,
    };

    let owned_ptr = &owned_lifetime as *const OwnedLifetime as *mut ();

    owned_drop_impl(owned_ptr);
}

