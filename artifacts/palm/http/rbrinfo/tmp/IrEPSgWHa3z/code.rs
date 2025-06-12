fn find<K>(&self, key: &K) -> Option<(usize, usize)>
    where
        K: Hash + Into<HeaderName> + ?Sized,
        HeaderName: PartialEq<K>,
    {
        if self.entries.is_empty() {
            return None;
        }

        let hash = hash_elem_using(&self.danger, key);
        let mask = self.mask;
        let mut probe = desired_pos(mask, hash);
        let mut dist = 0;

        probe_loop!(probe < self.indices.len(), {
            if let Some((i, entry_hash)) = self.indices[probe].resolve() {
                if dist > probe_distance(mask, entry_hash, probe) {
                    // give up when probe distance is too long
                    return None;
                } else if entry_hash == hash && self.entries[i].key == *key {
                    return Some((probe, i));
                }
            } else {
                return None;
            }

            dist += 1;
        });
    }