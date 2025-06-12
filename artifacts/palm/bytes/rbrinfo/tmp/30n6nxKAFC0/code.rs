fn from(slice: Box<[u8]>) -> Bytes {
        // Box<[u8]> doesn't contain a heap allocation for empty slices,
        // so the pointer isn't aligned enough for the KIND_VEC stashing to
        // work.
        if slice.is_empty() {
            return Bytes::new();
        }

        let len = slice.len();
        let ptr = Box::into_raw(slice) as *mut u8;

        if ptr as usize & 0x1 == 0 {
            let data = ptr_map(ptr, |addr| addr | KIND_VEC);
            Bytes {
                ptr,
                len,
                data: AtomicPtr::new(data.cast()),
                vtable: &PROMOTABLE_EVEN_VTABLE,
            }
        } else {
            Bytes {
                ptr,
                len,
                data: AtomicPtr::new(ptr.cast()),
                vtable: &PROMOTABLE_ODD_VTABLE,
            }
        }
    }