fn from(slice: &'a mut [u8]) -> Self {
        UninitSlice::new(slice)
    }