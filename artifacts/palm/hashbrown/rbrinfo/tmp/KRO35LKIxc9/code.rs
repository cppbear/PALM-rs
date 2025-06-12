unsafe fn ctrl(&self, index: usize) -> *mut Tag {
        debug_assert!(index < self.num_ctrl_bytes());
        // SAFETY: The caller must uphold the safety rules for the [`RawTableInner::ctrl`]
        self.ctrl.as_ptr().add(index).cast()
    }