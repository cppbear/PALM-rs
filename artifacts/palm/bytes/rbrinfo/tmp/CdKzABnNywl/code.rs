fn uninit_ref(slice: &[MaybeUninit<u8>]) -> &UninitSlice {
        unsafe { &*(slice as *const [MaybeUninit<u8>] as *const UninitSlice) }
    }