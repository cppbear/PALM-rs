fn from(bytes: BytesMut) -> Self {
        let kind = bytes.kind();
        let bytes = ManuallyDrop::new(bytes);

        let mut vec = if kind == KIND_VEC {
            unsafe {
                let off = bytes.get_vec_pos();
                rebuild_vec(bytes.ptr.as_ptr(), bytes.len, bytes.cap, off)
            }
        } else {
            let shared = bytes.data as *mut Shared;

            if unsafe { (*shared).is_unique() } {
                let vec = mem::replace(unsafe { &mut (*shared).vec }, Vec::new());

                unsafe { release_shared(shared) };

                vec
            } else {
                return ManuallyDrop::into_inner(bytes).deref().to_vec();
            }
        };

        let len = bytes.len;

        unsafe {
            ptr::copy(bytes.ptr.as_ptr(), vec.as_mut_ptr(), len);
            vec.set_len(len);
        }

        vec
    }