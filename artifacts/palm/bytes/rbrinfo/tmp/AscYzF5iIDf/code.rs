fn from(slice: &'a mut [MaybeUninit<u8>]) -> Self {
        UninitSlice::uninit(slice)
    }