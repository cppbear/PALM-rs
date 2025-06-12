fn from(vec: Vec<u8>) -> Bytes {
        let mut vec = ManuallyDrop::new(vec);
        let ptr = vec.as_mut_ptr();
        let len = vec.len();
        let cap = vec.capacity();

        // Avoid an extra allocation if possible.
        if len == cap {
            let vec = ManuallyDrop::into_inner(vec);
            return Bytes::from(vec.into_boxed_slice());
        }

        let shared = Box::new(Shared {
            buf: ptr,
            cap,
            ref_cnt: AtomicUsize::new(1),
        });

        let shared = Box::into_raw(shared);
        // The pointer should be aligned, so this assert should
        // always succeed.
        debug_assert!(
            0 == (shared as usize & KIND_MASK),
            "internal: Box<Shared> should have an aligned pointer",
        );
        Bytes {
            ptr,
            len,
            data: AtomicPtr::new(shared as _),
            vtable: &SHARED_VTABLE,
        }
    }