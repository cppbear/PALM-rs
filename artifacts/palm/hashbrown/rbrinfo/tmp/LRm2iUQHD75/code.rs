pub(crate) unsafe fn load_aligned(ptr: *const Tag) -> Self {
        // FIXME: use align_offset once it stabilizes
        debug_assert_eq!(ptr as usize & (mem::align_of::<Self>() - 1), 0);
        Group(x86::_mm_load_si128(ptr.cast()))
    }