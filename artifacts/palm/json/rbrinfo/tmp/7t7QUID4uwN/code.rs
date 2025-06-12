pub fn sort_keys(&mut self) {
        #[cfg(feature = "preserve_order")]
        self.map.sort_unstable_keys();
    }