fn add(&mut self) -> Option<StatePtr> {
        let si = self.table.len();
        if si > STATE_MAX as usize {
            return None;
        }
        self.table.extend(repeat(STATE_UNKNOWN).take(self.num_byte_classes));
        Some(usize_to_u32(si))
    }