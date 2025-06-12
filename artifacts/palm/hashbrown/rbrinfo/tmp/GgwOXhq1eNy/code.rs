fn num_ctrl_bytes(&self) -> usize {
        self.bucket_mask + 1 + Group::WIDTH
    }