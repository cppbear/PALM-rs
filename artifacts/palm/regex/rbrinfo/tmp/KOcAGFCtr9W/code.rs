fn replace_append(&mut self, caps: &Captures, dst: &mut Vec<u8>) {
        caps.expand(*self, dst);
    }