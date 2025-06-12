pub fn new(bytes: &'a mut [u8]) -> Self {
        Buf { bytes, offset: 0 }
    }