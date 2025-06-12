fn copy_to_slice(&mut self, dst: &mut [u8]) {
        self.try_copy_to_slice(dst)
            .unwrap_or_else(|error| panic_advance(&error));
    }