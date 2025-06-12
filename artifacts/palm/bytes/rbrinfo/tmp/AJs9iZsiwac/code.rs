pub unsafe fn as_uninit_slice_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        &mut self.0
    }