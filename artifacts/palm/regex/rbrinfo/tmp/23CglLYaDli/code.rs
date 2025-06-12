fn u8_class(&self, b: u8) -> usize {
        self.prog.byte_classes[b as usize] as usize
    }