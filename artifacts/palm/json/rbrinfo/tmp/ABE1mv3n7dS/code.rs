pub fn remove(self) -> Value {
        #[cfg(feature = "preserve_order")]
        return self.swap_remove();
        #[cfg(not(feature = "preserve_order"))]
        return self.occupied.remove();
    }