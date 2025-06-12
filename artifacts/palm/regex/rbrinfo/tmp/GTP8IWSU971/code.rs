fn replace_append(&mut self, caps: &Captures, dst: &mut Vec<u8>) {
        dst.extend_from_slice(&(*self)(caps));
    }