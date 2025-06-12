fn num_byte_classes(&self) -> usize {
        // We add 1 to account for the special EOF byte.
        (self.prog.byte_classes[255] as usize + 1) + 1
    }