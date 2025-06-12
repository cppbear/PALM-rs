unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
        raw::format32(self, result)
    }