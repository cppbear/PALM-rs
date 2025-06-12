fn increment_indices(&mut self, start: usize, end: usize) {
        // Use a heuristic between a full sweep vs. a `find()` for every shifted item.
        let shifted_entries = &self.entries[start..end];
        if shifted_entries.len() > self.indices.capacity() / 2 {
            // Shift all indices in range.
            for i in &mut *self.indices {
                if start <= *i && *i < end {
                    *i += 1;
                }
            }
        } else {
            // Find each entry in range to shift its index, updated in reverse so
            // we never have duplicated indices that might have a hash collision.
            for (i, entry) in (start..end).zip(shifted_entries).rev() {
                update_index(self.indices, entry.hash, i, i + 1);
            }
        }
    }