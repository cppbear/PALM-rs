pub unsafe fn from_raw_parts_mut<'a>(ptr: *mut u8, len: usize) -> &'a mut UninitSlice {
        let maybe_init: &mut [MaybeUninit<u8>] =
            core::slice::from_raw_parts_mut(ptr as *mut _, len);
        Self::uninit(maybe_init)
    }