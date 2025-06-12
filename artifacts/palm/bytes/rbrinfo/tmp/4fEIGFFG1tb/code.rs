pub fn from_owner<T>(owner: T) -> Self
    where
        T: AsRef<[u8]> + Send + 'static,
    {
        // Safety & Miri:
        // The ownership of `owner` is first transferred to the `Owned` wrapper and `Bytes` object.
        // This ensures that the owner is pinned in memory, allowing us to call `.as_ref()` safely
        // since the lifetime of the owner is controlled by the lifetime of the new `Bytes` object,
        // and the lifetime of the resulting borrowed `&[u8]` matches that of the owner.
        // Note that this remains safe so long as we only call `.as_ref()` once.
        //
        // There are some additional special considerations here:
        //   * We rely on Bytes's Drop impl to clean up memory should `.as_ref()` panic.
        //   * Setting the `ptr` and `len` on the bytes object last (after moving the owner to
        //     Bytes) allows Miri checks to pass since it avoids obtaining the `&[u8]` slice
        //     from a stack-owned Box.
        // More details on this: https://github.com/tokio-rs/bytes/pull/742/#discussion_r1813375863
        //                  and: https://github.com/tokio-rs/bytes/pull/742/#discussion_r1813316032

        let owned = Box::into_raw(Box::new(Owned {
            lifetime: OwnedLifetime {
                ref_cnt: AtomicUsize::new(1),
                drop: owned_box_and_drop::<T>,
            },
            owner,
        }));

        let mut ret = Bytes {
            ptr: NonNull::dangling().as_ptr(),
            len: 0,
            data: AtomicPtr::new(owned.cast()),
            vtable: &OWNED_VTABLE,
        };

        let buf = unsafe { &*owned }.owner.as_ref();
        ret.ptr = buf.as_ptr();
        ret.len = buf.len();

        ret
    }