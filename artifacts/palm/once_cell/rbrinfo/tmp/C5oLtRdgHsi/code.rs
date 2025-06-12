pub(crate) unsafe fn get_unchecked(&self) -> &T {
        debug_assert!(self.is_initialized());
        let slot = &*self.value.get();
        slot.as_ref().unwrap_unchecked()
    }