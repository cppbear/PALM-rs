fn write(&mut self, _: &[u8]) {
        unreachable!("TypeId calls write_u64");
    }