fn replace_append(&mut self, _: &Captures, dst: &mut Vec<u8>) {
        dst.extend_from_slice(self.0);
    }