pub fn len_utf8(self) -> usize {
        char::from_u32(self.0).map_or(0, |c| c.len_utf8())
    }