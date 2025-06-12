pub fn remove<K>(&mut self, key: K) -> Option<T>
    where
        K: AsHeaderName,
    {
        match key.find(self) {
            Some((probe, idx)) => {
                if let Some(links) = self.entries[idx].links {
                    self.remove_all_extra_values(links.next);
                }

                let entry = self.remove_found(probe, idx);

                Some(entry.value)
            }
            None => None,
        }
    }