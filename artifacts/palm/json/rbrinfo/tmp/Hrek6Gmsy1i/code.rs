pub fn remove_entry(self) -> (String, Value) {
        #[cfg(feature = "preserve_order")]
        return self.swap_remove_entry();
        #[cfg(not(feature = "preserve_order"))]
        return self.occupied.remove_entry();
    }