fn try_grow(&mut self, new_raw_cap: usize) -> Result<(), MaxSizeReached> {
        if new_raw_cap > MAX_SIZE {
            return Err(MaxSizeReached::new());
        }

        // find first ideally placed element -- start of cluster
        let mut first_ideal = 0;

        for (i, pos) in self.indices.iter().enumerate() {
            if let Some((_, entry_hash)) = pos.resolve() {
                if 0 == probe_distance(self.mask, entry_hash, i) {
                    first_ideal = i;
                    break;
                }
            }
        }

        // visit the entries in an order where we can simply reinsert them
        // into self.indices without any bucket stealing.
        let old_indices = mem::replace(
            &mut self.indices,
            vec![Pos::none(); new_raw_cap].into_boxed_slice(),
        );
        self.mask = new_raw_cap.wrapping_sub(1) as Size;

        for &pos in &old_indices[first_ideal..] {
            self.reinsert_entry_in_order(pos);
        }

        for &pos in &old_indices[..first_ideal] {
            self.reinsert_entry_in_order(pos);
        }

        // Reserve additional entry slots
        let more = self.capacity() - self.entries.len();
        self.entries.reserve_exact(more);
        Ok(())
    }