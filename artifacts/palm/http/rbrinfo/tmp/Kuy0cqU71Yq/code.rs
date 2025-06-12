fn reinsert_entry_in_order(&mut self, pos: Pos) {
        if let Some((_, entry_hash)) = pos.resolve() {
            // Find first empty bucket and insert there
            let mut probe = desired_pos(self.mask, entry_hash);

            probe_loop!(probe < self.indices.len(), {
                if self.indices[probe].resolve().is_none() {
                    // empty bucket, insert here
                    self.indices[probe] = pos;
                    return;
                }
            });
        }
    }