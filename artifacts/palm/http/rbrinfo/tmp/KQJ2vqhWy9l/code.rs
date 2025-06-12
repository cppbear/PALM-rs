fn rebuild(&mut self) {
        // Loop over all entries and re-insert them into the map
        'outer: for (index, entry) in self.entries.iter_mut().enumerate() {
            let hash = hash_elem_using(&self.danger, &entry.key);
            let mut probe = desired_pos(self.mask, hash);
            let mut dist = 0;

            // Update the entry's hash code
            entry.hash = hash;

            probe_loop!(probe < self.indices.len(), {
                if let Some((_, entry_hash)) = self.indices[probe].resolve() {
                    // if existing element probed less than us, swap
                    let their_dist = probe_distance(self.mask, entry_hash, probe);

                    if their_dist < dist {
                        // Robinhood
                        break;
                    }
                } else {
                    // Vacant slot
                    self.indices[probe] = Pos::new(index, hash);
                    continue 'outer;
                }

                dist += 1;
            });

            do_insert_phase_two(&mut self.indices, probe, Pos::new(index, hash));
        }
    }