fn has_prefix(&self) -> bool {
        !self.prog.is_reverse
        && !self.prog.prefixes.is_empty()
        && !self.prog.is_anchored_start
    }