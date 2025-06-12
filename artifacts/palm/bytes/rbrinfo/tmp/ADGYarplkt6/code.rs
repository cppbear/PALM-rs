pub fn new(slice: &mut [u8]) -> &mut UninitSlice {
        unsafe { &mut *(slice as *mut [u8] as *mut [MaybeUninit<u8>] as *mut UninitSlice) }
    }