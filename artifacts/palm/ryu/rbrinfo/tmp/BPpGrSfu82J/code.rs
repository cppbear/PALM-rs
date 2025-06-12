unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
        raw::format64(self, result)
    }