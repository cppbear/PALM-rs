pub fn new() -> Self {
        let bytes = [MaybeUninit::<u8>::uninit(); 24];
        Buffer { bytes }
    }