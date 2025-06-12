fn copy_to_bytes(&mut self, len: usize) -> Bytes {
        self.split_to(len).freeze()
    }