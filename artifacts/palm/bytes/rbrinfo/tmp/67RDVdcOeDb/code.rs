fn from(bytes: Bytes) -> Self {
        let bytes = ManuallyDrop::new(bytes);
        unsafe { (bytes.vtable.into_mut)(&bytes.data, bytes.ptr, bytes.len) }
    }