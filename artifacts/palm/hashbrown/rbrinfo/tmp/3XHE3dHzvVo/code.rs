pub(crate) unsafe fn load(ptr: *const Tag) -> Self {
        Group(x86::_mm_loadu_si128(ptr.cast()))
    }