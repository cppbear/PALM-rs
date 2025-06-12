pub(crate) unsafe fn store_aligned(self, ptr: *mut Tag) {
        // FIXME: use align_offset once it stabilizes
        debug_assert_eq!(ptr as usize & (mem::align_of::<Self>() - 1), 0);
        x86::_mm_store_si128(ptr.cast(), self.0);
    }