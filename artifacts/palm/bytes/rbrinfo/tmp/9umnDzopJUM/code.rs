unsafe fn shallow_clone_vec(
    atom: &AtomicPtr<()>,
    ptr: *const (),
    buf: *mut u8,
    offset: *const u8,
    len: usize,
) -> Bytes {
    // If the buffer is still tracked in a `Vec<u8>`. It is time to
    // promote the vec to an `Arc`. This could potentially be called
    // concurrently, so some care must be taken.

    // First, allocate a new `Shared` instance containing the
    // `Vec` fields. It's important to note that `ptr`, `len`,
    // and `cap` cannot be mutated without having `&mut self`.
    // This means that these fields will not be concurrently
    // updated and since the buffer hasn't been promoted to an
    // `Arc`, those three fields still are the components of the
    // vector.
    let shared = Box::new(Shared {
        buf,
        cap: offset_from(offset, buf) + len,
        // Initialize refcount to 2. One for this reference, and one
        // for the new clone that will be returned from
        // `shallow_clone`.
        ref_cnt: AtomicUsize::new(2),
    });

    let shared = Box::into_raw(shared);

    // The pointer should be aligned, so this assert should
    // always succeed.
    debug_assert!(
        0 == (shared as usize & KIND_MASK),
        "internal: Box<Shared> should have an aligned pointer",
    );

    // Try compare & swapping the pointer into the `arc` field.
    // `Release` is used synchronize with other threads that
    // will load the `arc` field.
    //
    // If the `compare_exchange` fails, then the thread lost the
    // race to promote the buffer to shared. The `Acquire`
    // ordering will synchronize with the `compare_exchange`
    // that happened in the other thread and the `Shared`
    // pointed to by `actual` will be visible.
    match atom.compare_exchange(ptr as _, shared as _, Ordering::AcqRel, Ordering::Acquire) {
        Ok(actual) => {
            debug_assert!(actual as usize == ptr as usize);
            // The upgrade was successful, the new handle can be
            // returned.
            Bytes {
                ptr: offset,
                len,
                data: AtomicPtr::new(shared as _),
                vtable: &SHARED_VTABLE,
            }
        }
        Err(actual) => {
            // The upgrade failed, a concurrent clone happened. Release
            // the allocation that was made in this thread, it will not
            // be needed.
            let shared = Box::from_raw(shared);
            mem::forget(*shared);

            // Buffer already promoted to shared storage, so increment ref
            // count.
            shallow_clone_arc(actual as _, offset, len)
        }
    }
}