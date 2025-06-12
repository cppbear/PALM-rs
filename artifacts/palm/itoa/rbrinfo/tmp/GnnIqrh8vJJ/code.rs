pub fn new() -> Buffer {
        let bytes = [MaybeUninit::<u8>::uninit(); i128::MAX_STR_LEN];
        Buffer { bytes }
    }