fn remove_found(&mut self, probe: usize, found: usize) -> Bucket<T> {
        // index `probe` and entry `found` is to be removed
        // use swap_remove, but then we need to update the index that points
        // to the other entry that has to move
        self.indices[probe] = Pos::none();
        let entry = self.entries.swap_remove(found);

        // correct index that points to the entry that had to swap places
        if let Some(entry) = self.entries.get(found) {
            // was not last element
            // examine new element in `found` and find it in indices
            let mut probe = desired_pos(self.mask, entry.hash);

            probe_loop!(probe < self.indices.len(), {
                if let Some((i, _)) = self.indices[probe].resolve() {
                    if i >= self.entries.len() {
                        // found it
                        self.indices[probe] = Pos::new(found, entry.hash);
                        break;
                    }
                }
            });

            // Update links
            if let Some(links) = entry.links {
                self.extra_values[links.next].prev = Link::Entry(found);
                self.extra_values[links.tail].next = Link::Entry(found);
            }
        }

        // backward shift deletion in self.indices
        // after probe, shift all non-ideally placed indices backward
        if !self.entries.is_empty() {
            let mut last_probe = probe;
            let mut probe = probe + 1;

            probe_loop!(probe < self.indices.len(), {
                if let Some((_, entry_hash)) = self.indices[probe].resolve() {
                    if probe_distance(self.mask, entry_hash, probe) > 0 {
                        self.indices[last_probe] = self.indices[probe];
                        self.indices[probe] = Pos::none();
                    } else {
                        break;
                    }
                } else {
                    break;
                }

                last_probe = probe;
            });
        }

        entry
    }