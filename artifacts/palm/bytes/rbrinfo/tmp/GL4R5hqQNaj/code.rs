fn from(bytes: Bytes) -> Vec<u8> {
        let bytes = ManuallyDrop::new(bytes);
        unsafe { (bytes.vtable.into_vec)(&bytes.data, bytes.ptr, bytes.len) }
    }