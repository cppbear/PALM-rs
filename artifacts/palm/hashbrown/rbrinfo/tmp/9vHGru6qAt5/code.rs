fn clear_no_drop(&mut self) {
        if !self.is_empty_singleton() {
            unsafe {
                self.ctrl(0)
                    .write_bytes(Tag::EMPTY.0, self.num_ctrl_bytes());
            }
        }
        self.items = 0;
        self.growth_left = bucket_mask_to_capacity(self.bucket_mask);
    }